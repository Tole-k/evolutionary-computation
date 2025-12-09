use evolutionary::hybrid_evolutionary;
use evolutionary::utils;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_hybrid() {
        let data: Vec<utils::DataPoint> = utils::load_data(&format!("../data/TSPA.csv"));
        let distance_matrix = utils::calculate_distance_matrix(&data);
        let path = hybrid_evolutionary::hybrid_algorithm(&data, 0, &distance_matrix);
        assert_eq!(path.len(), 100);
        let score = utils::check_solution(&path, &data, &distance_matrix);
        assert!(score < 73000.0);
        assert!(score > 68000.0);
    }
}
