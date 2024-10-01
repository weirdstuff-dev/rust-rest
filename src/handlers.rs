use crate::models::{Brand, NewBrand, DbPool};
use crate::schema::brands;
use actix_web::{get, post, put, delete, web, HttpResponse, Error};
use diesel::prelude::*;

#[post("/brands")]
pub async fn create_brand(pool: web::Data<DbPool>, new_brand: web::Json<NewBrand>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let brand = web::block(move || {
        diesel::insert_into(brands::table)
            .values(&new_brand.into_inner())
            .returning(Brand::as_returning())
            .get_result::<Brand>(&mut conn)
    }).await?;

    Ok(HttpResponse::Ok().json(brand.unwrap()))
}

#[get("/brands/{id}")]
pub async fn get_brand(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let brand = web::block(move || brands::table.find(id.into_inner()).first::<Brand>(&mut conn))
        .await?;

    Ok(HttpResponse::Ok().json(brand.unwrap()))
}

#[get("/brands")]
pub async fn get_all_brands(pool: web::Data<DbPool>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let brands = web::block(move || brands::table.load::<Brand>(&mut conn))
        .await?;

    Ok(HttpResponse::Ok().json(brands.unwrap()))
}

#[put("/brands/{id}")]
pub async fn update_brand(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    updated_brand: web::Json<NewBrand>,
) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let brand = web::block(move || {
        diesel::update(brands::table.find(id.into_inner()))
            .set(&updated_brand.into_inner())
            .get_result::<Brand>(&mut conn)
    })
        .await?;

    Ok(HttpResponse::Ok().json(brand.unwrap()))
}

#[delete("/brands/{id}")]
pub async fn delete_brand(pool: web::Data<DbPool>, id: web::Path<i32>) -> Result<HttpResponse, Error> {
    let mut conn = pool.get().expect("couldn't get db connection from pool");

    let result = web::block(move || {
        diesel::delete(brands::table.find(id.into_inner())).execute(&mut conn)
    }).await?;

    if result.unwrap() > 0 {
        Ok(HttpResponse::Ok().finish())
    } else {
        Ok(HttpResponse::NotFound().finish())
    }
}