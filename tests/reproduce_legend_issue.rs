//! Test to reproduce the legend issue with multiple indicators

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

#[test]
fn test_legend_with_multiple_indicators_country_styles() {
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

    assert!(result.is_ok(), "Chart creation should succeed");
    assert!(path.exists(), "SVG file should be created");
    
    let svg_content = std::fs::read_to_string(&path).unwrap();
    println!("SVG content length: {}", svg_content.len());
    
    // Check if legend contains indicator information
    let has_population = svg_content.contains("Population");
    let has_gdp = svg_content.contains("GDP");
    
    println!("Legend contains 'Population': {}", has_population);
    println!("Legend contains 'GDP': {}", has_gdp);
    
    // With multiple indicators, both should appear in legend
    assert!(has_population, "Legend should contain 'Population' for multiple indicators");
    assert!(has_gdp, "Legend should contain 'GDP' for multiple indicators");
}

#[test]
fn test_legend_with_multiple_indicators_no_country_styles() {
    let data = create_multi_indicator_data();
    let temp_file = NamedTempFile::new().unwrap();
    let path = temp_file.path().with_extension("svg");

    println!("Testing legend with country_styles=None...");
    let result = wbi_rs::viz::plot_chart(
        &data,
        &path,
        800,
        600,
        "en",
        LegendMode::Right,
        "Multiple Indicators Test - Normal",
        PlotKind::LinePoints,
        0.3,
        None, // no country styles
    );

    assert!(result.is_ok(), "Chart creation should succeed");
    assert!(path.exists(), "SVG file should be created");
    
    let svg_content = std::fs::read_to_string(&path).unwrap();
    
    // Check if legend contains indicator information
    let has_population = svg_content.contains("Population");
    let has_gdp = svg_content.contains("GDP");
    
    println!("Legend contains 'Population': {}", has_population);
    println!("Legend contains 'GDP': {}", has_gdp);
    
    // With multiple indicators, both should appear in legend
    assert!(has_population, "Legend should contain 'Population' for multiple indicators");
    assert!(has_gdp, "Legend should contain 'GDP' for multiple indicators");
}