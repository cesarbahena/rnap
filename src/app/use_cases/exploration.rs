use super::super::*;

impl Dnap {
    pub fn create_exploration_graph(
        &mut self,
        input: CreateExplorationGraph,
    ) -> Result<CreatedExplorationGraph, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;

        let allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.created_by,
            &input.promoter_gene_fqn,
        )?;
        let allele = self
            .alleles
            .get(&allele_id)
            .ok_or(DnapError::AlleleNotFound)?;
        self.require_locus_encoding(allele.locus_id, EncodingKind::Promoter)?;
        let promoter_locus = self
            .loci
            .get(&allele.locus_id)
            .cloned()
            .ok_or(DnapError::AlleleNotFound)?;
        let name = require_text(input.name, DnapError::BlankExplorationGraphName)?;
        let now = SystemTime::now();
        let graph = ExplorationGraph {
            id: self.allocate_exploration_graph_id(),
            promoter_locus_id: promoter_locus.id,
            name,
            created_by: input.created_by,
            created_at: now,
            updated_at: now,
        };

        self.exploration_graphs.insert(graph.id, graph.clone());

        Ok(CreatedExplorationGraph {
            graph,
            promoter_locus,
        })
    }

    pub fn add_exploration_node(
        &mut self,
        input: AddExplorationNode,
    ) -> Result<AddedExplorationNode, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;
        let graph = self
            .exploration_graphs
            .get(&input.graph_id)
            .cloned()
            .ok_or(DnapError::ExplorationGraphNotFound)?;
        let promoter_locus = self
            .loci
            .get(&graph.promoter_locus_id)
            .ok_or(DnapError::ExplorationGraphNotFound)?;
        if promoter_locus.insulator_id != input.insulator_id
            || promoter_locus.genome_id != input.genome_id
        {
            return Err(DnapError::ExplorationGraphNotFound);
        }

        let erna_locus_name =
            require_text(input.erna_locus_name, DnapError::BlankExplorationNodeName)?;
        let mut created_erna = None;
        let erna_locus = match self.find_locus_by_encoding(
            input.insulator_id,
            input.genome_id,
            EncodingKind::ERna,
            &erna_locus_name,
        ) {
            Some(locus) => locus.clone(),
            None => {
                let family_abbreviation = input
                    .erna_family_abbreviation
                    .clone()
                    .ok_or(DnapError::ExplorationNodeErnaFamilyRequired)?;
                let mutated = self.mutate_new(MutateNew {
                    insulator_id: input.insulator_id,
                    genome_id: input.genome_id,
                    gene_family_abbreviation: family_abbreviation,
                    locus_name: erna_locus_name.clone(),
                    mutations: Vec::new(),
                    causes: Vec::new(),
                    created_by: input.created_by,
                })?;
                self.require_locus_encoding(mutated.locus.id, EncodingKind::ERna)?;
                let locus = mutated.locus.clone();
                created_erna = Some(mutated);
                locus
            }
        };
        self.require_locus_encoding(erna_locus.id, EncodingKind::ERna)?;
        let label = match input.label {
            Some(label) => require_text(label, DnapError::BlankExplorationNodeName)?,
            None => erna_locus.name.clone(),
        };
        let now = SystemTime::now();
        let node = ExplorationNode {
            id: self.allocate_exploration_node_id(),
            graph_id: input.graph_id,
            erna_locus_id: erna_locus.id,
            label,
            position_x: input.position_x,
            position_y: input.position_y,
            created_by: input.created_by,
            created_at: now,
            updated_at: now,
        };

        self.exploration_nodes.insert(node.id, node.clone());

        Ok(AddedExplorationNode {
            node,
            erna_locus,
            created_erna,
        })
    }

    pub fn add_exploration_edge(
        &mut self,
        input: AddExplorationEdge,
    ) -> Result<ExplorationEdge, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.created_by, input.insulator_id)?;
        let graph = self
            .exploration_graphs
            .get(&input.graph_id)
            .ok_or(DnapError::ExplorationGraphNotFound)?;
        let promoter_locus = self
            .loci
            .get(&graph.promoter_locus_id)
            .ok_or(DnapError::ExplorationGraphNotFound)?;
        if promoter_locus.insulator_id != input.insulator_id
            || promoter_locus.genome_id != input.genome_id
        {
            return Err(DnapError::ExplorationGraphNotFound);
        }
        let from = self
            .exploration_nodes
            .get(&input.from_node_id)
            .ok_or(DnapError::ExplorationNodeNotFound)?;
        let to = self
            .exploration_nodes
            .get(&input.to_node_id)
            .ok_or(DnapError::ExplorationNodeNotFound)?;
        if from.graph_id != input.graph_id || to.graph_id != input.graph_id {
            return Err(DnapError::ExplorationEdgeCrossGraph);
        }
        let label = input
            .label
            .map(|label| require_text(label, DnapError::BlankExplorationEdgeLabel))
            .transpose()?;
        let now = SystemTime::now();
        let edge = ExplorationEdge {
            id: self.allocate_exploration_edge_id(),
            graph_id: input.graph_id,
            from_node_id: input.from_node_id,
            to_node_id: input.to_node_id,
            label,
            created_by: input.created_by,
            created_at: now,
        };

        self.exploration_edges.insert(edge.id, edge.clone());
        Ok(edge)
    }

    pub fn attach_enhancer_promoter(
        &mut self,
        input: AttachEnhancerPromoter,
    ) -> Result<EnhancerContext, DnapError> {
        self.require_insulator(input.insulator_id)?;
        self.require_genome_in_insulator(input.genome_id, input.insulator_id)?;
        self.require_tf_in_insulator(input.updated_by, input.insulator_id)?;

        let enhancer_allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.updated_by,
            &input.enhancer_gene_fqn,
        )?;
        let promoter_allele_id = self.resolve_active_allele_id(
            input.insulator_id,
            input.genome_id,
            input.updated_by,
            &input.promoter_gene_fqn,
        )?;
        let enhancer_locus_id = self
            .alleles
            .get(&enhancer_allele_id)
            .map(|allele| allele.locus_id)
            .ok_or(DnapError::AlleleNotFound)?;
        let promoter_locus_id = self
            .alleles
            .get(&promoter_allele_id)
            .map(|allele| allele.locus_id)
            .ok_or(DnapError::AlleleNotFound)?;
        if !self.locus_has_encoding(enhancer_locus_id, EncodingKind::Enhancer) {
            return Err(DnapError::EnhancerContextEnhancerRequired);
        }
        if !self.locus_has_encoding(promoter_locus_id, EncodingKind::Promoter) {
            return Err(DnapError::EnhancerContextPromoterRequired);
        }

        let context = EnhancerContext {
            enhancer_locus_id,
            promoter_locus_id,
            updated_by: input.updated_by,
            updated_at: SystemTime::now(),
        };
        self.enhancer_contexts
            .insert(enhancer_locus_id, context.clone());
        Ok(context)
    }
}
