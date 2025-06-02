use crate::booking;

use actix_web::{get, Responder};
use actix_web::web::Path;


#[get("/api/free")]
pub async fn free() -> impl Responder {
    booking::with(|booking| format!("{{ free: {} }}", booking.free()))
}

#[get("/api/book/{day}")]
pub async fn book(day: usize) -> impl Responder {
    booking::with(|booking| {
        match booking.book(day.deref()) {
            Some(index) => format!("{{ booked: {} }}", index),
            None => String::from("{}"),
        }
    })
}


