/////////////////////////////////////////////////////
// Start 3rd-party library
//
struct StateMachine;

impl StateMachine {
    // The 3rd-party library dictates that self is a mutable reference.
    fn notify_client(&mut self, client: &mut dyn Client) {
        client.mutate();
    }
}

trait Client {
    fn mutate(&mut self);
}

//
// End 3rd-party library
/////////////////////////////////////////////////////

struct MyThing {
    state_machine_from_3rd_party_lib: StateMachine,
    data: usize,
}

impl MyThing {
    fn advance_state_machine(&mut self) {
        let machine = &mut self.state_machine_from_3rd_party_lib;
        machine.notify_client(self);
    }
}

impl Client for MyThing {
    fn mutate(&mut self) {
        self.data += 1;
    }
}

pub fn main() {
    let mut thing = MyThing {
        state_machine_from_3rd_party_lib: StateMachine {},
        data: 0,
    };
    thing.advance_state_machine();
    println!("{}", thing.data);
}
