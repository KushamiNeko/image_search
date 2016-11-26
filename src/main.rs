extern crate image;

use std::path::Path;
use std::path::PathBuf;
use image::GenericImage;
use image::Pixel;

const DATABASE: &'static str = r#"app_resource\database"#;

// const DATABASE_FILTERED: &'static str = r#"E:\Programming project\Rust\image_search\app_resource\database_filtered"#;
// const SEARCH_SOURCE: &'static str = r#"E:\Programming project\Rust\image_search\app_resource\search_source"#;

// const SEARCH_TEST: &'static str = r#"E:\Programming project\Rust\image_search\app_resource\search_source\6.jpg"#;
// const COMPARE_TEST: &'static str = r#"E:\Programming project\Rust\image_search\app_resource\database\ASS_BS_148.jpg"#;

const PIXEL_COMPARE_TEST: &'static str = r#"app_resource\pixel_operation_debug\comparasion_region.png"#;
const PIXEL_MULTIPLY_TEST: &'static str = r#"app_resource\pixel_operation_debug\multiply_debug.png"#;

const SIZE_BORDER_WHITE_THRESHOLD: f64 = 245.0;

const COMPARASION_SEARCH_RANGE: u32 = 10;
const Y_WIDTH_COMPARASION_RANGE: u32 = 7;
const START_COLLECT_THRESHOLD: u32 = 50;

const TEMPLATE_BLACK_THRESHOLD: f64 = 190.0;
const IMAGE_COMPARASION_WHITE_THRESHOLD: f64 = 210.0;
const COMPARE_DIFERENCE_TRESHOLD: u32 = 300;

const IMAGE_SIZE_CHECKING_INITIALIZE_VALUE: u32 = 9999;
const ALPHA_OUTPUT: u8 = 255;

// 411 232 1.jpg

fn search_image_dimension_check(image_path: &PathBuf) -> (Vec<(u32, u32)>, u32, u32, u32, u32) {
	let image_source = image::open(image_path).unwrap();
	let (width, height) = image_source.dimensions();

	let mut compare_test = image::DynamicImage::new_rgb8(width, height);

	let mut white_compare_vec: Vec<(u32, u32)> = Vec::new();

	let mut start_x: u32 = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;
	let mut end_x: u32 = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;

	let mut start_y: u32 = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;
	let mut end_y: u32 = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;

	let mut approximate_width: u32 = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;

	let mut compare_match_x;
	let mut compare_match_y = 0;
	let mut start_collect = false;

	let red = image::Rgba{
		data: [255, 0, 0, ALPHA_OUTPUT] as [u8;4],
	};

	for x in 0..width {
		if end_x != 9999 && x > end_x {
			continue;
		} else if approximate_width != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE
		&& start_x != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE {
			if x > approximate_width + start_x {
				continue;
			}
		}

		if approximate_width != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE && x > approximate_width - 10 {
			compare_match_x = 0;
			for y in start_y..end_y - 1 {
				let source_pixel = image_source.get_pixel(x, y);
				let (r, g, b, _) = source_pixel.channels4();
				let average: f64 = (r as f64 + g as f64 + b as f64) / 3.0;

				if average < SIZE_BORDER_WHITE_THRESHOLD {
					compare_match_x += 1;
				}
			}

			if compare_match_x < START_COLLECT_THRESHOLD {
				if end_x == IMAGE_SIZE_CHECKING_INITIALIZE_VALUE {
					end_x = x;
				}
			}
		}

		if start_collect == false {

			for y in 0..height{
				let source_pixel = image_source.get_pixel(x, y);
				let (r, g, b, _) = source_pixel.channels4();
				let average: f64 = (r as f64 + g as f64 + b as f64) / 3.0;

				if average < SIZE_BORDER_WHITE_THRESHOLD {
					compare_match_y += 1;
					if start_y == IMAGE_SIZE_CHECKING_INITIALIZE_VALUE {
						start_x = x;
						let mut y_width_compare = 0;
						for i in 1..COMPARASION_SEARCH_RANGE {
							let pixel = image_source.get_pixel(x + i, y);
							let (cr, cg, cb, _) = pixel.channels4();
							let average_c: f64 = (cr as f64 + cg as f64 + cb as f64) / 3.0;

							if average_c < SIZE_BORDER_WHITE_THRESHOLD {
								y_width_compare += 1;
							}
						}
						if y_width_compare >= Y_WIDTH_COMPARASION_RANGE {
							start_y = y;
						} else {
							start_y = y + 1;
						}
					}
				} else {
					if compare_match_y > START_COLLECT_THRESHOLD {
						if end_y == IMAGE_SIZE_CHECKING_INITIALIZE_VALUE {
							let mut y_width_compare = 0;
							for i in 1..COMPARASION_SEARCH_RANGE {
								let pixel = image_source.get_pixel(x + i, y - 1);
								let (cr, cg, cb, _) = pixel.channels4();
								let average_c: f64 = (cr as f64 + cg as f64 + cb as f64) / 3.0;

								if average_c < SIZE_BORDER_WHITE_THRESHOLD {
									y_width_compare += 1;
								}
							}
							if y_width_compare >= Y_WIDTH_COMPARASION_RANGE {
								end_y = y;
							} else {
								end_y = y - 1;
							}
						}
					} else {
						compare_match_y = 0;
						start_y = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;
						start_x = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;
						end_y = IMAGE_SIZE_CHECKING_INITIALIZE_VALUE;
					}
				}

				if end_y != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE && start_y != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE
				&& start_x != IMAGE_SIZE_CHECKING_INITIALIZE_VALUE {
					start_collect = true;
					let y_ratio: f64 = ((end_y - 1) as f64 - start_y as f64 + 1.0) / 9.0;
					approximate_width = (y_ratio * 16.0) as u32 + 1;
				}
			}
		}

		if start_collect == true {
			for y in start_y..end_y {
				let source_pixel = image_source.get_pixel(x, y);
				let (r, g, b, _) = source_pixel.channels4();
				let average: f64 = (r as f64 + g as f64 + b as f64) / 3.0;

				if average > IMAGE_COMPARASION_WHITE_THRESHOLD {
					if x != end_x {
						white_compare_vec.push((x, y));
						compare_test.put_pixel(x, y, source_pixel);
					}
				} else {
					compare_test.put_pixel(x, y, red);
				}
			}
		}
	}

	let ref mut out_put_file = std::fs::File::create(&Path::new(PIXEL_COMPARE_TEST)).unwrap();
	let _ = compare_test.save(out_put_file, image::PNG);
	(white_compare_vec, start_x, end_x, start_y, end_y)
}

fn comparasion_template_generation(source: &PathBuf, template: &PathBuf, width: &u32, height: &u32,
 source_start_x: &u32, source_start_y: &u32) -> Box<image::DynamicImage> {
	let image_source = image::open(source).unwrap();
	let template_source = image::open(template).unwrap();
	let template_resize = template_source.resize_exact(*width, *height, image::FilterType::CatmullRom);

	let comparasion_template = image_multiplication(&image_source, &template_resize, width, height, source_start_x, source_start_y);
	comparasion_template
}

fn image_multiplication(source: &image::DynamicImage, template: &image::DynamicImage, width: &u32, height: &u32,
	source_start_x: &u32, source_start_y: &u32) -> Box<image::DynamicImage> {
	let mut comparasion_template = Box::new(image::DynamicImage::new_rgb8(*width, *height));
	for x in 0..*width {
		for y in 0..*height {
			let source_pixel = source.get_pixel(x + *source_start_x, y + *source_start_y);
			let (sr, sg, sb, _) = source_pixel.channels4();

			let template_pixel = template.get_pixel(x, y);
			let (tr, tg, tb, _) = template_pixel.channels4();
			let averaget: f64 = (tr as f64 + tg as f64 + tb as f64) / 3.0;

			if averaget < TEMPLATE_BLACK_THRESHOLD {
				let (nsr, nsg, nsb) = color_normalize(sr, sg, sb);

				let nor = nsr * 0f64;
				let nog = nsg * 0f64;
				let nob = nsb * 0f64;

				let (or, og, ob) = color_to_8bits(nor, nog, nob);
				let final_pixel = image::Rgba{
					data: [or, og, ob, ALPHA_OUTPUT] as [u8;4],
				};

				(*comparasion_template).put_pixel(x, y, final_pixel);
			} else {
				(*comparasion_template).put_pixel(x, y, source_pixel);
			}
		}
	}
	comparasion_template
}

fn color_normalize(r: u8, g: u8, b: u8) -> (f64, f64, f64) {
	let nr = r as f64 / 255.0;
	let ng = g as f64 / 255.0;
	let nb = b as f64 / 255.0;
	(nr, ng, nb)
}

fn color_to_8bits(r: f64, g: f64, b: f64) -> (u8, u8, u8) {
	let rr = (r * 255.0).round();
	let or: u8 = match rr {
		num @ 0.0...255.0 => num as u8,
		out @ _ => {
			if out > 255.0 {
				255
			} else if out < 0.0 {
				0
			} else {
				unreachable!("{}", "impossible to reach!");
			}
		},
	};

	let rg = (g * 255.0).round();
	let og: u8 = match rg {
		num @ 0.0...255.0 => num as u8,
		out @ _ => {
			if out > 255.0 {
				255
			} else if out < 0.0 {
				0
			} else {
				unreachable!("{}", "impossible to reach!");
			}
		},
	};

	let rb = (b * 255.0).round();
	let ob: u8 = match rb {
		num @ 0.0...255.0 => num as u8,
		out @ _ => {
			if out > 255.0 {
				255
			} else if out < 0.0 {
				0
			} else {
				unreachable!("{}", "impossible to reach!");
			}
		},
	};
	(or, og, ob)
}

fn image_comparasion(search_source_path: &PathBuf, compare_template_path: &PathBuf, width: &u32, height: &u32,
	source_start_x: &u32, source_start_y: &u32, search_region: &Vec<(u32, u32)>) -> Option<String> {

	let mut comparasion_image = comparasion_template_generation(search_source_path, compare_template_path, width, height,
	 source_start_x, source_start_y);

	let mut difference_accumulation = 0;

	for item in search_region.iter() {
		let (x, y) = *item;
		let pixel = (*comparasion_image).get_pixel(x - *source_start_x, y - *source_start_y);
		let (r, g, b, _) = pixel.channels4();

		let blue = image::Rgba{
			data: [0, 0, 255, ALPHA_OUTPUT] as [u8;4],
		};

		let green = image::Rgba{
			data: [0, 255, 0, ALPHA_OUTPUT] as [u8;4],
		};

		if r == 0 && g == 0 && b == 0 {
			(*comparasion_image).put_pixel(x - *source_start_x, y - *source_start_y, blue);
			difference_accumulation += 1;
		} else {
			(*comparasion_image).put_pixel(x - *source_start_x, y - *source_start_y, green);
		}

		if difference_accumulation > COMPARE_DIFERENCE_TRESHOLD {
			return None;
		}
	}

	let ref mut out_put_file = std::fs::File::create(&Path::new(PIXEL_MULTIPLY_TEST)).unwrap();
	let _ = comparasion_image.save(out_put_file, image::PNG);

	let file_name_os = compare_template_path.file_name().unwrap();
	let file_name_str = file_name_os.to_str().unwrap();

	Some(file_name_str.to_string())

	// green is comparasion_region and blue is black pixel inside the region.
}

fn main() {
	let mut search_file_path = PathBuf::new();
	loop {
		println!("{:?}", "Please enter a file to search or leave it blank to leave: ");

		let mut user_input = String::new();
		let _ = std::io::stdin().read_line(&mut user_input);

		let user_input_clean = user_input.trim().replace("\"", "");
		if &user_input_clean == "" {
			println!("{:?}", "Thanks for trying Rust Image Search program!");
			return;
		}
		let user_path = Path::new(&user_input_clean);
		let user_path_metadata = std::fs::metadata(user_path);
		match user_path_metadata {
			Ok(result) => {
				if result.is_file() {
					search_file_path.push(user_path);
					break;
				} else {
					println!("{:?}", "Please enter a file not a directory!");
					continue;
				}
			},
			Err(_) => {
				println!("{:?}", "Please enter a valid file path!");
				continue;
			},
		}
	}

    let (search_region, start_x, end_x, start_y, end_y) = search_image_dimension_check(&search_file_path);
    let image_width = end_x - start_x;
    let image_height = end_y - start_y;

    let database_read = std::fs::read_dir(Path::new(DATABASE)).unwrap();

    for i in database_read {
    	let item = i.unwrap();
    	let template_path = item.path();

    	let file_name = image_comparasion(&search_file_path, &template_path, &image_width, &image_height,
    		&start_x, &start_y, &search_region);
	    match file_name {
	    	Some(name) => {
	    		println!("Match: {}", name);
	    		return;
	    	},
	    	None => {
	    		println!("Not match: {}", template_path.file_name().unwrap().to_str().unwrap());
	    		continue;
	    	},
	    }
    }

}
