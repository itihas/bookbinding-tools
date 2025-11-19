use bookbinding_tools::{Block, Layout, PaperSize};
use clap::Parser;
use pdfium_render::prelude::*;

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

    /// Size of the paper you're printing on
    #[arg(short = 'g', long = "galleysize", value_enum, default_value = "a4")]
    output_paper_size: PaperSize,

    /// Folio layout
    #[arg(short = 'f', long = "folio", default_value_t, group = "layout")]
    folio: bool,

    /// Quarto layout
    #[arg(short = 'q', long = "quarto", group = "layout")]
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

    let pdfium = Pdfium::default();
    let mut doc = pdfium.load_pdf_from_file(args.input_file.as_str(), None)?;

    let block = Block {
        output_paper_size: args.output_paper_size,
        layout: Layout::Folio, // TODO: figure out arg parsing for this in a little bit
        signature_size: args.signature_size,
        margins: bookbinding_tools::MarginSet {
            printer: args.printer_margin,
            cutting: args.cutting_margin,
            binding: args.binding_margin,
            annotation: args.annotation_margin,
        },
    };

    doc.pages_mut()
        .tile_into_new_document(1, 2, PdfPagePaperSize::a4().landscape())?
        .save_to_file("test/test_file.pdf")?;
    Ok(())
}
