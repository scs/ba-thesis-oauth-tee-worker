extern crate sgx_tstd as std;
use std::string::String;

/**
 * html_elements.rs
 * 
 * This file contains the HTML elements used by the server.
 **/


/*******************
 * Dynamic contents
 *******************/

pub fn html_resource_table(resource_content: &str, token: &str, expiry: &str) -> String {
  return format!("
  <table>
    <tr>
      <th>Resource</th>
      <th>Token</th>
      <th>Expiry</th>
    </tr>
    <tr>
      <td>{}</td>
      <td>{}</td>
      <td>{}</td>
    </tr>
  </table>
  ", resource_content, token, expiry);
}

pub fn html_authorization_prompt(error: &str) -> String {
  return format!("{}\n\n{}\n</body>\n</html>", HTML_AUTHORIZATION_PROMPT, error);
}


/*******************
 * HTML Elements
 *******************/

pub static HTML_404: &str ="
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Hello!</title>
  </head>
  <body>
    <h1>Oops!</h1>
    <p>Sorry, I don't know what you're asking for.</p>
  </body>
</html>
";

pub static HTML_RESOURCE: &str = "
<p>This is the content of the protected resource.</p>
";

pub static HTML_ACCESS_DENIED: &str = "
<!DOCTYPE html>
<html lang='en'>
    Access Denied
</html>
";

pub static HTML_RESOURCE_HEADER: &str = "
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Protected Resource</title>
  </head>
  <body>
";

pub static HTML_RESOURCE_FOOTER: &str = "
  </body>
</html>
";

pub static HTML_AUTHORIZATION_PROMPT: &str = "
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Authorization Prompt</title>
  </head>
  <body>
    <h1>Authorization Prompt</h1>
    <p>It seems like you don't have a valid access token, to let me access your protected resource please fill out the following fields:</p>
    <form action='/authorize' method='post'>
      <label for='username'>Username</label>
      <input type='text' id='username' name='username' required>
      <label for='password'>Password</label>
      <input type='password' id='password' name='password' required>
      <input type='submit' value='Authorize'>
    </form>
";
