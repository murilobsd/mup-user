pub mod user_entity;
pub mod user_mapper;
pub mod user_persistence_adapter;
pub(crate) mod user_repository;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
