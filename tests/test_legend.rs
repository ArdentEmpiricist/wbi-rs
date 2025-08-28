#[cfg(test)]
mod tests {
    use std::path::PathBuf;
    use wbi_rs::models::DataPoint;
    use wbi_rs::viz::{self, LegendMode, PlotKind};

    #[test]
    fn test_legend_development() {
        let mut points = Vec::new();
        
        // Series 1: DEU
        for (y, v) in [(2019, 1.0), (2020, 2.0), (2021, 3.0)] {
            points.push(DataPoint {
                indicator_id: "GDP".into(),
                indicator_name: "GDP Growth Rate".into(),
                country_id: "DE".into(),
                country_name: "Germany".into(),
                country_iso3: "DEU".into(),
                year: y,
                value: Some(v),
                unit: None,
                obs_status: None,
                decimal: None,
            });
        }
        
        // Series 2: USA  
        for (y, v) in [(2019, 2.0), (2020, 2.5), (2021, 3.5)] {
            points.push(DataPoint {
                indicator_id: "GDP".into(),
                indicator_name: "GDP Growth Rate".into(),
                country_id: "US".into(),
                country_name: "United States".into(),
                country_iso3: "USA".into(),
                year: y,
                value: Some(v),
                unit: None,
                obs_status: None,
                decimal: None,
            });
        }
        
        // Series 3: FRA
        for (y, v) in [(2019, 1.5), (2020, 2.2), (2021, 3.2)] {
            points.push(DataPoint {
                indicator_id: "GDP".into(),
                indicator_name: "GDP Growth Rate".into(),
                country_id: "FR".into(),
                country_name: "France".into(),
                country_iso3: "FRA".into(),
                year: y,
                value: Some(v),
                unit: None,
                obs_status: None,
                decimal: None,
            });
        }
        
        // Test Top legend - should spread items across x-axis
        let path = PathBuf::from("/tmp/legend_top_development.svg");
        viz::plot_chart(
            &points,
            &path,
            900,
            520,
            "en",
            LegendMode::Top,
            "Legend Test - Top Mode",
            PlotKind::LinePoints,
            0.3,
            None,
        ).unwrap();
        println!("Created: {}", path.display());
        
        // Test Bottom legend
        let path = PathBuf::from("/tmp/legend_bottom_development.svg");
        viz::plot_chart(
            &points,
            &path,
            900,
            520,
            "en",
            LegendMode::Bottom,
            "Legend Test - Bottom Mode", 
            PlotKind::LinePoints,
            0.3,
            None,
        ).unwrap();
        println!("Created: {}", path.display());
    }
}