use std::sync::{LazyLock, Arc, Mutex, MutexGuard};

static BOOKING: LazyLock<Arc<Mutex<Booking>>> = LazyLock::new(|| Arc::new(Mutex::new(Booking::new())));


#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Status {
    Reserved,
    Free,
}

#[derive(Debug, Clone, Copy)]
pub struct Parking {
    status: Status,
}

impl Parking {
    pub fn new() -> Parking {
        Parking {
            status: Status::Free,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Day {
    parkings: Vec<Parking>,
}

impl Day {
    pub fn new() -> Day {
        let parkings = (0..22)
            .map(|_| Parking::new())
            .collect::<Vec<Parking>>();

        Day {
            parkings,
        }
    }
}

pub struct Booking {
    days: Vec<Day>,
}

impl Booking {
    pub fn new() -> Booking {
        let days = (0..365)
            .map(|_| Day::new())
            .collect::<Vec<Day>>();

        Booking {
            days,
        }
    }

    pub fn free(&self, day: usize) -> usize {
        self.days[day].parkings.iter()
            .filter(|parking| parking.status == Status::Free)
            .count()
    }

    pub fn book(&mut self, day: usize) -> Option<usize> {
        match self.days[day].parkings.iter().enumerate().filter(|(_, parking)| parking.status == Status::Free).next() {
            Some((index, _)) => {
                self.days[day].parkings[index].status = Status::Reserved;

                Some(index)
            },
            None => None,
        }
    }
}

pub fn with<R>(f: impl Fn(&mut MutexGuard<Booking>) -> R) -> R {
    f(&mut BOOKING.lock().expect("failed to lock"))
}


