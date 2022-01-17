use owo_colors::{OwoColorize, Rgb};


pub fn interpolate_result_color(result: u32, max: u32) -> String {
	let interpolation = (f64::from(result) / f64::from(max) * 256.) as u8;
	let red = 255 - interpolation;
	let green = interpolation;
	let color = Rgb(red, green, 0);
	result.color(color).to_string()
}
