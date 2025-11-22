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
    output_file: String,

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

#[derive(Clone, Debug)]
pub struct Layout {
    rows: u8,
    cols: u8,
    signature_size: u16,
    papersize: PdfPagePaperSize,
    layout: Vec<(usize, bool)>,
}

impl Layout {
    pub fn get_page_order(
        self: &Self,
        signature_size: u16,
    ) -> Result<Vec<(u16, bool)>, std::io::Error> {
        let pages_in_galley = self.layout.len() as u16;
        if signature_size % pages_in_galley != 0 {
            return Err(std::io::Error::new(std::io::ErrorKind::InvalidInput, format!("signature size isn't divisible by {}. Can't assemble a signature this size out of the chosen layout.", pages_in_galley)));
        }

        // e.g. with folio layout when size = 12,
        // layout_count = 3
        // pivot = 6
        let galley_count = signature_size / pages_in_galley;
        let pivot = signature_size / 2;
        let mut indices: Vec<(u16, bool)> = Vec::new();

        // i = 0, 1, 2
        // result 5, 6, 7, 4  ,  3, 8, 9, 2  ,  1, 10 , 11, 0

        for i in 0..galley_count {
            let this_signature: Vec<u16> = std::iter::chain(
                // bottom_half
                (0..(pages_in_galley / 2))
                    .rev()
                    .map(|x| pivot - (pages_in_galley / 2) * i - x - 1),
                // top half
                (0..(pages_in_galley / 2)).map(|x| pivot + (pages_in_galley / 2) * i + x),
            )
            .collect();

            for (j, verso) in &self.layout {
                indices.push((this_signature[*j], *verso))
            }
        }
        Ok(indices)
    }

    fn folio_layout() -> Layout {
        Layout {
            rows: 1,
            cols: 2,
            signature_size: 32,
            papersize: PdfPagePaperSize::Landscape(PdfPagePaperStandardSize::A4),
            layout: vec![(3, false), (0, false), (1, false), (2, false)],
        }
    }

    fn quarto_layout() -> Layout {
        Layout {
            rows: 2,
            cols: 2,
            signature_size: 32,
            papersize: PdfPagePaperSize::Portrait(PdfPagePaperStandardSize::A4),
            layout: vec![
                (4, true),
                (3, true),
                (7, false),
                (0, false),
                (2, true),
                (5, true),
                (1, false),
                (6, false),
            ],
        }
    }
}

fn assemble_target_doc(
    doc: &PdfDocument,
    target_doc: &mut PdfDocument,
    layout: &Layout,
) -> Result<(), Box<dyn std::error::Error>> {
    let indices = layout.get_page_order(layout.signature_size)?;

    let default_paper_size = doc
        .pages()
        .page_size(1)
        .map(|p| PdfPagePaperSize::from_points(p.width(), p.height()))?;

    let mut mark = 1;
    let last = doc.pages().len();

    while mark <= last {
        for (index, verso) in indices.iter() {
            let point = target_doc.pages().len();
            if mark + index > last {
                target_doc
                    .pages_mut()
                    .create_page_at_end(default_paper_size)?;
                dbg!(point, mark + index, last);
            } else {
                target_doc
                    .pages_mut()
                    .copy_page_from_document(&doc, mark + index - 1, point)?;
                dbg!(mark + index, point);
            }
            if *verso {
                target_doc
                    .pages_mut()
                    .get(point)?
                    .set_rotation(PdfPageRenderRotation::Degrees180);
            }
        }

        mark += layout.signature_size;
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let pdfium = Pdfium::default();
    let source_doc = pdfium.load_pdf_from_file(args.input_file.as_str(), None)?;

    let mut layout: Layout = if args.quarto {
	Layout::quarto_layout()
    } else {
        Layout::folio_layout()
    };
    layout.signature_size = args.signature_size;
    let mut target_doc: PdfDocument<'_> = pdfium.create_new_pdf()?;
    assemble_target_doc(&source_doc, &mut target_doc, &layout)?;
    target_doc
        .pages()
        .tile_into_new_document(layout.rows, layout.cols, layout.clone().papersize)?
        .save_to_file(&args.output_file)?;
    Ok(())
}
