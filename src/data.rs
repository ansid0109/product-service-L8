use crate::model::Product;
use crate::configuration::Settings;

pub fn fetch_products(_settings: &Settings) -> Vec<Product> {
    initial_products()
}

pub fn initial_products() -> Vec<Product> {
    vec![
        Product {
            id: 1,
            name: "UltraHD 55\" 4K Smart TV".to_string(),
            price: 499.99,
            description: "Bring movies, sports, and gaming to life with this 55-inch 4K Smart TV featuring HDR support, built-in streaming apps, and voice control compatibility.".to_string(),
            image: "/tv-55-4k.jpg".to_string()
        },
        Product {
            id: 2,
            name: "Noise-Canceling Wireless Headphones".to_string(),
            price: 249.99,
            description: "Enjoy immersive sound with active noise cancellation, all-day comfort, and up to 35 hours of battery life for travel, work, and entertainment.".to_string(),
            image: "/wireless-headphones.jpg".to_string()
        },
        Product {
            id: 3,
            name: "Gaming Laptop 15.6\" RTX Edition".to_string(),
            price: 1299.99,
            description: "Power through modern games and creative workloads with a high-refresh display, dedicated graphics, fast SSD storage, and advanced cooling.".to_string(),
            image: "/gaming-laptop.jpg".to_string()
        },
        Product {
            id: 4,
            name: "French Door Smart Refrigerator".to_string(),
            price: 1799.99,
            description: "Keep groceries fresh with spacious storage, adjustable shelving, and Wi-Fi controls that let you monitor and adjust settings from your phone.".to_string(),
            image: "/smart-refrigerator.jpg".to_string()
        },
        Product {
            id: 5,
            name: "Front-Load Washer with Steam".to_string(),
            price: 849.99,
            description: "Tackle tough laundry loads with multiple wash cycles, steam cleaning options, and energy-efficient performance designed for modern homes.".to_string(),
            image: "/front-load-washer.jpg".to_string()
        },
        Product {
            id: 6,
            name: "Electric Dryer with Sensor Dry".to_string(),
            price: 799.99,
            description: "Dry clothes evenly with moisture sensors that help prevent over-drying while offering quick-dry, delicate, and heavy-duty cycle options.".to_string(),
            image: "/electric-dryer.jpg".to_string()
        },
        Product {
            id: 7,
            name: "Countertop Air Fryer Oven".to_string(),
            price: 159.99,
            description: "Cook crispy meals with less oil using this multi-function air fryer oven with presets for fries, chicken, vegetables, and reheating leftovers.".to_string(),
            image: "/air-fryer-oven.jpg".to_string()
        },
        Product {
            id: 8,
            name: "Robot Vacuum with Smart Mapping".to_string(),
            price: 329.99,
            description: "Automate daily cleaning with smart navigation, app scheduling, and room mapping that helps the robot vacuum avoid obstacles and clean efficiently.".to_string(),
            image: "/robot-vacuum.jpg".to_string()
        },
        Product {
            id: 9,
            name: "Wi-Fi 7 Mesh Router 3-Pack".to_string(),
            price: 449.99,
            description: "Eliminate dead zones with whole-home Wi-Fi coverage, fast multi-gig performance, and simple app-based setup for streaming, gaming, and smart devices.".to_string(),
            image: "/wifi7-mesh-router.jpg".to_string()
        },
        Product {
            id: 10,
            name: "Stainless Steel Dishwasher".to_string(),
            price: 699.99,
            description: "Clean more dishes with quieter operation, adjustable racks, and targeted spray zones that handle everything from glassware to heavy cookware.".to_string(),
            image: "/dishwasher-stainless.jpg".to_string()
        }
    ]
}