use std::path::PathBuf;
use wbi_rs::models::DataPoint;
use wbi_rs::viz::{self, LegendMode, PlotKind};

fn sample_points_single_indicator() -> Vec<DataPoint> {
    vec![
        DataPoint {
            indicator_id: "SP.POP.TOTL".into(),
            indicator_name: "Population, total".into(),
            country_id: "US".into(),
            country_name: "United States".into(),
            country_iso3: "USA".into(),
            year: 2020,
            value: Some(331000000.0),
            unit: None,
            obs_status: None,
            decimal: None,
        },
    ]
}

#[test]
fn test_title_derivation_with_gui_default() {
    let tmp = std::env::temp_dir();
    let path: PathBuf = tmp.join("test_gui_title_derivation.svg");
    let points = sample_points_single_indicator();
    
    // This is what the GUI now passes when plot_title is empty
    let gui_default_title = "World Bank Indicator(s)";
    
    let result = viz::plot_chart(
        &points,
        &path,
        800,
        600,
        "en",
        LegendMode::Bottom,
        gui_default_title,
        PlotKind::Line,
        0.3,
        None,
    );
    
    assert!(result.is_ok(), "plot_chart should succeed");
    
    // Check if file was created
    let content = std::fs::read_to_string(&path).expect("file to exist");
    
    // The title should be derived from the indicator name, not the default
    assert!(content.contains("Population, total"), 
           "SVG should contain the derived title 'Population, total'");
    
    // Clean up
    std::fs::remove_file(&path).ok();
}

#[test]
fn test_custom_title_not_overridden() {
    let tmp = std::env::temp_dir();
    let path: PathBuf = tmp.join("test_custom_title.svg");
    let points = sample_points_single_indicator();
    
    let custom_title = "My Custom Chart Title";
    
    let result = viz::plot_chart(
        &points,
        &path,
        800,
        600,
        "en",
        LegendMode::Bottom,
        custom_title,
        PlotKind::Line,
        0.3,
        None,
    );
    
    assert!(result.is_ok(), "plot_chart should succeed");
    
    // Check if file was created
    let content = std::fs::read_to_string(&path).expect("file to exist");
    
    // The custom title should be preserved
    assert!(content.contains("My Custom Chart Title"), 
           "SVG should contain the custom title");
    
    // Clean up
    std::fs::remove_file(&path).ok();
}