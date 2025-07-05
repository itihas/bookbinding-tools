use clap::Parser;
use lopdf::Document;
use bookbinding_tools::{Block, PaperSize, Layout};

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
    #[arg(short = 'g', long = "galleysize", value_enum, default_value="a4")]
    galley_size: PaperSize,

    /// Folio layout
    #[arg(short = 'f', long = "folio", default_value_t, group = "layout")]
    folio: bool,

    /// Quarto layout
    #[arg(short = 'q', long= "quarto", group = "layout")]
    quarto: bool,
    
    /// Pad final signature with blank pages
    #[arg(short = 'p', long = "pad")]
    pad: bool,

    /// Generate separate PDFs for odd/even pages
    #[arg(short = 'S', long = "split-pdfs")]
    split_pdfs: bool,

    /// Output file
    #[arg(short = 'o', long = "output")]
    output_file: Option<String>,

    /// Binding margin (inner edge)
    #[arg(long = "binding-margin", default_value = "10")]
    binding_margin: f64, // mm

    /// Cutting margin (trim allowance)
    /// deckles are cool, but sure have a cutting margin if you like
    #[arg(long = "cutting-margin", default_value = "0")]
    cutting_margin: f64, // mm

    /// Annotation margin (space for notes on the outer edge)
    #[arg(long = "annotation-margin", default_value = "0")]
    annotation_margin: f64, // mm

    /// Use PDF's crop box, if it has one, to remove existing margins
    #[arg(long = "try-crop-input-margins")]
    crop_existing: bool,

    /// Printer margin (unprintable area)
    #[arg(long = "printer-margin", default_value = "3")]
    printer_margin: f64, // mm

    // /// Prefer layout that minimizes manual cuts
    // #[arg(long = "minimize-cuts")]
    // minimize_cuts: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let doc = Document::load(args.input_file)?;


    let block = Block {
	galley: args.galley_size,
	layout: Layout::Folio,	// TODO: figure out arg parsing for this in a little bit
	signature_size: args.signature_size,
	margins: bookbinding_tools::MarginSet {
	    printer: args.printer_margin,
	    cutting: args.cutting_margin,
	    binding: args.binding_margin,
	    annotation: args.annotation_margin
	}
    };
    println!("Block struct: {:?}", block);
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
