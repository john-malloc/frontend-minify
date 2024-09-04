# FRONTEND-MINIFY

**Bandwith is not free**, minify what you ship to the user.

1TB of bandwith on aws will cost you around 90$, on average extreme minification reduce your code size by 45%, this means saving 11.25kg of bell peppers (in italy 🇮🇹) or 40.5$ if do not count your money in bell peppers.

## How to use

Run `python3 frontend-minify.py` in the root directory of your project.

It will recursively minify all of the html, css and js files.
By default an extreme minification will be applied, it is the best choice if you do not have multiline strings that should not be formatted.

If you have files that contains multiline strings that should not be formatted run `python3 frontend-minify.py --exclude-extreme /path/to/file1 /path/to/file2`.

## Contribution

Contributions and feedback are welcome.