use super::{part_7_references, part_8_control_structures};

pub mod part_10_scoped_threads;
pub mod part_11_thread_parking;
pub mod part_12_part_1_async_await;
pub mod part_12_part_2_async_await_tasks;
pub mod part_12_part_3_asyn_await_select;
pub mod part_12_part_4_async_await_join;
pub mod part_1_thread_basics;
pub mod part_2_multiple_threads_and_ownership;
pub mod part_3_channels;
pub mod part_4_recieve_as_iterator;
pub mod part_5_mutiple_sender_threads;
pub mod part_6_reciever_waits_for_all_transmitters_to_drop;
pub mod part_7_sharing_state;
pub mod part_8_passing_mutexes_between_threads;
pub mod part_9_sync_through_barrier;
pub fn main() {
    // part_1_thread_basics::main();
    // part_2_multiple_threads_and_ownership::main();
    // part_3_channels::main();
    // part_4_recieve_as_iterator::main();
    // part_5_mutiple_sender_threads::main();
    // part_6_reciever_waits_for_all_transmitters_to_drop::main();
    // part_7_sharing_state::main();
    // part_8_passing_mutexes_between_threads::main();
    // part_9_sync_through_barrier::main();
    // part_10_scoped_threads::main();
    // part_11_thread_parking::main();
    // part_12_part_1_async_await::main();
    // part_12_part_2_async_await_tasks::main();
    // part_12_part_3_asyn_await_select::main();
    part_12_part_4_async_await_join::main();
}
