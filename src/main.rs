use clap::Parser;
use lopdf::Document;

// in points
const A4_DIMS: (f64, f64) = (595.28, 841.89);
const A5_DIMS: (f64, f64) = (419.53, 595.28);
const A6_DIMS: (f64, f64) = (297.64, 419.53);
const A7_DIMS: (f64, f64) = (209.76, 297.64);

enum PaperSize {
    A4,
    A5,
    A6,
    A7,
    Custom(Length, Length),
}

struct Length(f64);

impl PaperSize {
    fn get_size(&self) -> (Length, Length) {
        let (w, h) = match self {
            PaperSize::A4 => A4_DIMS,
            PaperSize::A5 => A5_DIMS,
            PaperSize::A6 => A6_DIMS,
            PaperSize::A7 => A7_DIMS,
            PaperSize::Custom(w, h) => (w.0, h.0),
        };
        (Length(w), Length(h))
    }
}

#[derive(Parser)]
#[command(name = "galley")]
#[command(
    about = "Galley maker tool. Given a pdf, outputs pdf(s) of galley(s) that you can print and fold into book signatures."
)]
struct Args {
    /// Input PDF file
    input_file: String,

    /// Signature size
    #[arg(short = 's', long = "signature", default_value = "16")]
    signature_size: u32,

    /// Size of the galley (paper you're printing on)
    #[arg(short = 'g', long = "galleysize", default_value = "A4")]
    galley_size: String,

    /// Size of the folio (final book size)
    #[arg(short = 'f', long = "foliosize", default_value = "A5")]
    folio_size: String,

    /// Pad final signature with blank pages
    #[arg(short = 'p', long = "pad")]
    pad: bool,

    /// Generate separate PDFs for odd/even pages
    #[arg(short = 'S', long = "split-pdfs")]
    split_pdfs: bool,

    /// Output file
    #[arg(short = 'o', long = "output")]
    output_file: Option<String>,

    /// Use landscape orientation for folio
    #[arg(short = 'l', long = "landscape")]
    folio_landscape: bool,
    /// Binding margin (inner edge)
    #[arg(long = "binding-margin", default_value = "10")]
    binding_margin: f64, // mm

    /// Cutting margin (trim allowance)
    /// deckles are cool, but sure have a cutting margin if you like
    #[arg(long = "cutting-margin", default_value = "3")]
    cutting_margin: f64, // mm

    /// Annotation margin (space for notes)
    #[arg(long = "annotation-margin", default_value = "0")]
    annotation_margin: f64, // mm

    /// Use PDF's crop box to remove existing margins
    #[arg(long = "crop-margins")]
    crop_existing: bool,

    /// Printer margin (unprintable area)
    /// My EPSON L130 has a 3mm print margin, so that's the default here.
    #[arg(long = "printer-margin", default_value = "3")]
    printer_margin: f64, // mm

    /// Prefer layout that minimizes manual cuts
    #[arg(long = "minimize-cuts")]
    minimize_cuts: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let doc = Document::load(args.input_file)?;

    // Get basic info
    let pages = doc.get_pages();
    println!("Page count: {}", pages.len());

    // Get dimensions of first page
    for (page_num, page_id) in pages.iter().take(1) {
        if let Ok(page_obj) = doc.get_object(*page_id) {
            if let Ok(page_dict) = page_obj.as_dict() {
                if let Ok(media_box) = page_dict.get(b"MediaBox") {
                    if let Ok(media_array) = media_box.as_array() {
                        if media_array.len() >= 4 {
                            let width = media_array[2].as_float()? - media_array[0].as_float()?;
                            let height = media_array[3].as_float()? - media_array[1].as_float()?;
                            println!("Page {}: {}x{} points", page_num, width, height);
                        }
                    }
                }
                if let Ok(content_box) = page_dict
                    .get(b"TrimBox")
                    .or_else(|_| page_dict.get(b"CropBox"))
                    .or_else(|_| page_dict.get(b"MediaBox"))
                {
                    if let Ok(media_array) = content_box.as_array() {
                        if media_array.len() >= 4 {
                            let width = media_array[2].as_float()? - media_array[0].as_float()?;
                            let height = media_array[3].as_float()? - media_array[1].as_float()?;
                            println!(
                                "Page {}: content size {}x{} points",
                                page_num, width, height
                            );
                        }
                    }
                }
            }
        }
    }
    Ok(())
}
