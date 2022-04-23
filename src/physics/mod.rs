use nalgebra::Vector2;
use nalgebra::Vector3;
use rapier2d::{prelude::{PhysicsPipeline, BroadPhase, NarrowPhase, RigidBodySet, ColliderSet, JointSet, CCDSolver, ChannelEventCollector, ContactEvent, IntersectionEvent, QueryPipeline, IslandManager, IntegrationParameters}, crossbeam::channel::Receiver};

pub struct PhysicsWorld {
    pipeline: PhysicsPipeline,
    gravity: Vector2<f32>,
    integration_parameters: IntegrationParameters,
    broad_phase: BroadPhase,
    narrow_phase: NarrowPhase,
    pub bodies: RigidBodySet,
    pub colliders: ColliderSet,
    joints: JointSet,
    ccd_solver: CCDSolver,
    event_handler: ChannelEventCollector,

    contact_event: Receiver<ContactEvent>,
    intersection_event: Receiver<IntersectionEvent>,

    pub query_pipeline: QueryPipeline,
    pub island_manager: IslandManager,
}
impl PhysicsWorld {
    pub fn new() -> Self {
        let collider_set = ColliderSet::new();
        let body_set = RigidBodySet::new();
        let (contact_send, contact_recv) = crossbeam::channel::unbounded();
        let (intersection_send, intersection_recv) = crossbeam::channel::unbounded();
        let event_handler = ChannelEventCollector::new(intersection_send, contact_send);

        let mut out = PhysicsWorld {
            pipeline: PhysicsPipeline::new(),
            gravity: Vector2::new(0.0f32, -9.81 * 4.0),
            integration_parameters: IntegrationParameters::default(),
            broad_phase: BroadPhase::new(),
            narrow_phase: NarrowPhase::new(),
            bodies: body_set,
            colliders: collider_set,
            joints: JointSet::new(),
            ccd_solver: CCDSolver::new(),

            event_handler: event_handler,
            intersection_event: intersection_recv,
            contact_event: contact_recv,
            query_pipeline: QueryPipeline::new(),
            island_manager: IslandManager::new(),

        };

        out
    }

    pub fn step(&mut self) {
        //        self.frame_timer = self.frame_timer + crate::utils::frametime();
        //        if Instant::now() - last_step > Duration::from_nanos(16666666) {
        // if self.frame_timer >= self.update_rate {
        let delta = 69f32;
        let dt = 1.0 / 60.0;

        // while delta > 0.0 {

        // }

        self.integration_parameters.set_inv_dt(delta);
        self.pipeline.step(
            &self.gravity,
            &self.integration_parameters,
            &mut self.island_manager,
            &mut self.broad_phase,
            &mut self.narrow_phase,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joints,
            &mut self.ccd_solver,
            &(),
            &self.event_handler,
        );
        // self.frame_timer = 0.0;
        // }

        self.query_pipeline
            .update(&self.island_manager, &self.bodies, &self.colliders);
    }

}
