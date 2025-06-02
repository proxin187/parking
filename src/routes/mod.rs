use crate::booking;

use actix_web::{get, Responder};
use actix_web::web::Path;


#[get("/api/free/{day}")]
pub async fn free(day: Path<usize>) -> impl Responder {
    let day = day.into_inner();

    booking::with(|booking| format!("{{ free: {} }}", booking.free(day)))
}

#[get("/api/book/{day}")]
pub async fn book(day: Path<usize>) -> impl Responder {
    let day = day.into_inner();

    booking::with(|booking| {
        match booking.book(day) {
            Some(index) => format!("{{ booked: {} }}", index),
            None => String::from("{}"),
        }
    })
}


