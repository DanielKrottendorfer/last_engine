use cgmath::Vector3;
use chained_component_system::chained_component_system;

use std::{sync::{*, atomic::AtomicBool}, thread::{JoinHandle, self}};
use core::sync::atomic::Ordering::SeqCst;


chained_component_system!(
    components{
        ve: Vector3<f32>,
    };

    entities{
        Ve(ve),
    };

    global_systems{
        VeSys(ve),
    };
);