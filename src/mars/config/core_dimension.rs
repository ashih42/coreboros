#[derive(Clone, Copy, Eq, PartialEq)]
pub enum CoreDimension {
    Pico,
    Nano,
    Micro,
    Mini,
    Small,
    Medium,
    Large,
}

impl CoreDimension {
    pub fn list_all_values() -> Box<[Self]> {
        Box::new([
            Self::Pico,
            Self::Nano,
            Self::Micro,
            Self::Mini,
            Self::Small,
            Self::Medium,
            Self::Large,
        ])
    }

    #[inline]
    pub const fn as_str(&self) -> &str {
        match self {
            Self::Pico => "Pico (5 x 4)",
            Self::Nano => "Nano (10 x 8)",
            Self::Micro => "Micro  (20 x 16)",
            Self::Mini => "Mini (40 x 32)",
            Self::Small => "Small (50 x 40)",
            Self::Medium => "Medium (80 x 64)",
            Self::Large => "Large (100 x 80)",
        }
    }

    #[inline]
    pub const fn as_size(&self) -> usize {
        let (width, height) = self.as_grid_dimensions();

        width * height
    }

    #[inline]
    pub const fn as_grid_dimensions(&self) -> (usize, usize) {
        match self {
            Self::Pico => (5, 4),
            Self::Nano => (10, 8),
            Self::Micro => (20, 16),
            Self::Mini => (40, 32),
            Self::Small => (50, 40),
            Self::Medium => (80, 64),
            Self::Large => (100, 80),
        }
    }
}

// #[derive(Clone, Eq, PartialEq)]
// pub struct CoreDimension {
//     pub name: String,
//     pub size: usize,
//     pub width: usize,
//     pub height: usize,
// }

// impl CoreDimension {
//     pub fn generate_presets() -> Vec<Self> {
//         vec![
//             Self::nano(),
//             Self::micro(),
//             Self::mini(),
//             Self::small(),
//             Self::medium(),
//             Self::large(),
//         ]
//     }

//     pub fn nano() -> Self {
//         Self {
//             name: "Nano (10 x 8)".to_owned(),
//             size: 80,
//             width: 10,
//             height: 8,
//         }
//     }

//     fn micro() -> Self {
//         Self {
//             name: "Micro  (20 x 16)".to_owned(),
//             size: 320,
//             width: 20,
//             height: 16,
//         }
//     }

//     fn mini() -> Self {
//         Self {
//             name: "Mini (40 x 32)".to_owned(),
//             size: 1280,
//             width: 40,
//             height: 32,
//         }
//     }

//     fn small() -> Self {
//         Self {
//             name: "Small (50 x 40)".to_owned(),
//             size: 2000,
//             width: 50,
//             height: 40,
//         }
//     }

//     fn medium() -> Self {
//         Self {
//             name: "Medium (80 x 64)".to_owned(),
//             size: 5120,
//             width: 80,
//             height: 64,
//         }
//     }

//     fn large() -> Self {
//         Self {
//             name: "Large (100 x 80)".to_owned(),
//             size: 8000,
//             width: 100,
//             height: 80,
//         }
//     }
// }

// // pub enum CoreDimension {
// //     Tiny,
// //     Medium,
// //     Large,
// // }

// // impl CoreDimension {
// //     pub fn as_size(&self) {
// //         match self {
// //             Self::Tiny => 80,
// //             Self::Medium => 80,
// //             Self::Large => 80,
// //         }
// //     }
// // }
