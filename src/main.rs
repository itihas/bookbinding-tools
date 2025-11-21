use bookbinding_tools::PaperSize;
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
    signature_size: u16,

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
    #[arg(short = 'p', long = "pad", default_value_t)]
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

fn signature_shuffle(size: u16) -> Result<Vec<u16>, std::io::Error> {
    if size % 4 != 0 {
        return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, "signature size isn't divisible by 4. Can't assemble a signature this size out of folios."));
    }

    // e.g. when size = 12,
    // folio_count = 3
    // pivot = 6
    let folio_count = size / 4;
    let pivot = size / 2;
    let mut indices: Vec<u16> = Vec::new();

    // i = 0, 1, 2
    // result 5, 6, 7, 4  ,  3, 8, 9, 2  ,  1, 10 , 11, 0
    for i in 0..folio_count {
        indices.push(pivot - 2 * i - 1);
        indices.push(pivot + 2 * i);
        indices.push(pivot + 2 * i + 1);
        indices.push(pivot - 2 * i - 2);
    }
    Ok(indices)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let pdfium = Pdfium::default();
    let doc = pdfium.load_pdf_from_file(args.input_file.as_str(), None)?;

    let signature_size = args.signature_size;
    let indices = signature_shuffle(signature_size)?;

    let mut target_doc = pdfium.create_new_pdf()?;
    let mut mark: u16 = 1;
    let last = doc.pages().len();
    while mark < last {
        let paper_size = doc
            .pages()
            .page_size(mark)
            .map(|p| PdfPagePaperSize::from_points(p.width(), p.height()))?;

        for index in indices.iter() {
            let point = target_doc.pages().len();
            if mark + index > last {
                target_doc.pages_mut().create_page_at_end(paper_size)?;
                println!(
                    "blank page at point {:?}: couldn't find a page at {:?} + {:?}",
                    point, mark, index
                );
            } else {
                target_doc
                    .pages_mut()
                    .copy_page_from_document(&doc, mark + index - 1, point)?;
                println!("source page {:?} at point {:?}", mark + index, point);
            }
        }

        mark += signature_size;
    }

    target_doc
        .pages_mut()
        .tile_into_new_document(1, 2, PdfPagePaperSize::a4().landscape())?
        .save_to_file("test/test_file.pdf")?;
    Ok(())
}
