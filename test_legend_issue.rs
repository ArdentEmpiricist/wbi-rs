use tempfile::NamedTempFile;
use wbi_rs::models::DataPoint;
use wbi_rs::viz::{LegendMode, PlotKind};

fn create_multi_indicator_data() -> Vec<DataPoint> {
    vec![
        DataPoint {
            country_iso3: "USA".to_string(),
            country_name: "United States".to_string(),
            country_id: "US".to_string(),
            indicator_id: "SP.POP.TOTL".to_string(),
            indicator_name: "Population, total".to_string(),
            year: 2020,
            value: Some(331_000_000.0),
            unit: Some("people".to_string()),
            obs_status: None,
            decimal: None,
        },
        DataPoint {
            country_iso3: "USA".to_string(),
            country_name: "United States".to_string(),
            country_id: "US".to_string(),
            indicator_id: "NY.GDP.MKTP.CD".to_string(),
            indicator_name: "GDP (current US$)".to_string(),
            year: 2020,
            value: Some(20_950_000_000_000.0),
            unit: Some("current US$".to_string()),
            obs_status: None,
            decimal: None,
        },
        DataPoint {
            country_iso3: "DEU".to_string(),
            country_name: "Germany".to_string(),
            country_id: "DE".to_string(),
            indicator_id: "SP.POP.TOTL".to_string(),
            indicator_name: "Population, total".to_string(),
            year: 2020,
            value: Some(83_240_525.0),
            unit: Some("people".to_string()),
            obs_status: None,
            decimal: None,
        },
        DataPoint {
            country_iso3: "DEU".to_string(),
            country_name: "Germany".to_string(),
            country_id: "DE".to_string(),
            indicator_id: "NY.GDP.MKTP.CD".to_string(),
            indicator_name: "GDP (current US$)".to_string(),
            year: 2020,
            value: Some(3_846_000_000_000.0),
            unit: Some("current US$".to_string()),
            obs_status: None,
            decimal: None,
        },
    ]
}

fn main() {
    let data = create_multi_indicator_data();
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().with_extension("svg");

    println!("Testing legend with country_styles=Some(true)...");
    // Test with country styles enabled - this should show the issue
    let result = wbi_rs::viz::plot_chart(
        &data,
        &path,
        800,
        600,
        "en",
        LegendMode::Right,
        "Multiple Indicators Test - Country Styles",
        PlotKind::LinePoints,
        0.3,
        Some(true), // enable country styles - this causes the issue
    );

    match result {
        Ok(_) => {
            println!("Chart created successfully at: {:?}", path);
            let svg_content = std::fs::read_to_string(&path).unwrap();
            println!("SVG content length: {}", svg_content.len());
            
            // Check if legend contains indicator information
            if svg_content.contains("Population") {
                println!("✓ Legend contains 'Population'");
            } else {
                println!("✗ Legend missing 'Population'");
            }
            
            if svg_content.contains("GDP") {
                println!("✓ Legend contains 'GDP'");
            } else {
                println!("✗ Legend missing 'GDP'");
            }
        }
        Err(e) => println!("Error creating chart: {}", e),
    }

    // Test without country styles
    let path2 = temp_file.path().with_extension("svg2");
    println!("\nTesting legend with country_styles=None...");
    let result2 = wbi_rs::viz::plot_chart(
        &data,
        &path2,
        800,
        600,
        "en",
        LegendMode::Right,
        "Multiple Indicators Test - Normal",
        PlotKind::LinePoints,
        0.3,
        None, // no country styles
    );

    match result2 {
        Ok(_) => {
            println!("Chart created successfully at: {:?}", path2);
            let svg_content = std::fs::read_to_string(&path2).unwrap();
            
            // Check if legend contains indicator information
            if svg_content.contains("Population") {
                println!("✓ Legend contains 'Population'");
            } else {
                println!("✗ Legend missing 'Population'");
            }
            
            if svg_content.contains("GDP") {
                println!("✓ Legend contains 'GDP'");
            } else {
                println!("✗ Legend missing 'GDP'");
            }
        }
        Err(e) => println!("Error creating chart: {}", e),
    }
}