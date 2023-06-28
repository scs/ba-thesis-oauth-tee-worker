pub static HTML_HELLO: &str ="
<!DOCTYPE html>
<html lang='en'>
  <head>
    <meta charset='utf-8'>
    <title>Hello!</title>
  </head>
  <body>
    <h1>Hello!</h1>
    <p>Hi from Rust</p>
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
    <p>An application is requesting authorization to access your resources.</p>
    <p>Do you want to grant access?</p>
    <form action='/authorize?response_type=code&client_id=LocalClient' method='post'>
      <button type='submit'>Grant Access</button>
      <button type='button' onclick='window.close();'>Deny Access</button>
    </form>
  </body>
</html>
";

pub static HTML_RESOURCE_CONTENT: &str = "
<!DOCTYPE html>
<html lang='en'>
  <head>
      <title>Resource Content</title>
  </head>
  <body>
      <h1>Welcome to the Resource!</h1>
      <p>This is the content of the protected resource.</p>
  </body>
</html>
";

pub static HTML_DENY_TEXT: &str = "
<!DOCTYPE html>
<html lang='en'>
    This page should be accessed via an oauth token from the client in the example. Click
    <a href=\"http://localhost:7878/authorize?response_type=code&client_id=LocalClient&redirect_uri=localhost:7878/resource\">
    here</a> to begin the authorization process.
</html>
";

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