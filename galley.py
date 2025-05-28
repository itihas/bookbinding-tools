import sys, getopt



def __main__():
    try:
        args, values = getopts.getopt(sys.argv[1:], options, long_options)

        for arg, value in args:
            if arg in ("-h", "--help"):
                print('''
                galley [OPTS] INPUT_FILE
                Galley maker tool. Given a pdf, outputs pdf(s) of galley(s) that you can print and fold into book signatures.
                -s --signature signature size
                -g --galleysize size of the galley, i.e. the paper you're printing on
                -f --foliosize size of the folio, i.e. the final size of the book
                -p --pad whether to pad the final signature with blank pages to make it the target signature size
                -S --split-pdfs whether to generate two pdfs. one for odd pages and one for even. helps with printing.
                -o --output output file. If -S is also passed, will output <filename>.odds.pdf and <filename>.evens.pdf.
                ''')
