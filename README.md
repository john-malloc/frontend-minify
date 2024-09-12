<h1>FRONTEND-MINIFY</h1>

<p><bold>Bandwith is not free<bold>, minify what you ship to the user.</p>

<p>1TB of bandwith on aws will cost you around 90$, on average extreme minification reduce your code size by 45%, this means saving 11.25kg of bell peppers (in italy 🇮🇹) or 40.5$ if do not count your money in bell peppers.</p>

<h2>HOW TO BUILD</h2>

<h3>Run in the root directory of this project.</h3>

<code>cargo build --release</code>

<h3>Then move the ./target/release/frontend-minify binary in the root of your project</h3>

<h2>HOW TO USE</h2>

<h3>Run in the root directory of your project.</h3>

<code>./frontend-minify</code>

<table>
  <tr>
    <th>Flag</th>
    <th>Parameters</th>
    <th>Description</th>
  </tr>
  <tr>
    <td>--exclude-extreme</td>
    <td>path/to/file1 <br> path/to/file2</td>
    <td>for files that contains multiline strings that should not be formatted run</td>
  </tr>
  <tr>
    <td>--exclude-license</td>
    <td>path/to/file1 lines_to_skip_file1 <br> path/to/file2 lines_to_skip_file</td>
    <td>for files that have a license at the top that you want to maintain</td>
  </tr>
</table> 

<h2>Contribution</h2>

<p>Contributions and feedback are welcome.</p>

<h3>Convention to commit</h3>

<ul>
  <li>feat: feature</li><br>
  <li>fix: bug fixes</li><br>
  <li>docs: changes to the documentation</li><br>
  <li>style: style or formatting change</li><br>
  <li>perf: improves code performance</li><br>
  <li>test: test a feature</li><br>
</ul>