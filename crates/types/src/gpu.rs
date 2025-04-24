use std::marker::PhantomData;

struct Idle;
struct Allocated;
struct Preparing;
struct Computing;
struct Finalizing;
struct Suspended;
struct Faulty;

struct GPU<State> {
    id: u32,
    _phantom: PhantomData<State>,
}

impl GPU<Idle> {
    fn allocate(self, _task: Task) -> GPU<Allocated> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Allocated> {
    fn start_preparing(self) -> GPU<Preparing> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
    fn deallocate(self) -> GPU<Idle> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Preparing> {
    fn start_computing(self) -> GPU<Computing> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Computing> {
    fn suspend(self) -> GPU<Suspended> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
    fn finish_computing(self) -> GPU<Finalizing> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Suspended> {
    fn resume(self) -> GPU<Computing> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Finalizing> {
    fn complete(self) -> GPU<Idle> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

impl GPU<Faulty> {
    fn repair(self) -> GPU<Idle> {
        GPU {
            id: self.id,
            _phantom: PhantomData,
        }
    }
}

// Dummy Task type
struct Task;