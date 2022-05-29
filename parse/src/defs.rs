use super::*;

pub fn parse_defs(f: &mut File, it: &mut Tokens) -> Result<(), ParseError> {
  while it.hasnext() {
    parse_def(f, it)?;
  }
  Ok(())
}

fn parse_def(f: &mut File, it: &mut Tokens) -> Result<(), ParseError> {
  let next = it.next()?;

  // Import statement
  if next.kind == TokenKind::Ident("import".to_string()) {
    if let TokenKind::String(name) = it.next()?.kind {
      if TokenKind::Ident("as".to_string()) == it.next()?.kind { // Check for as statement
        if let TokenKind::Ident(pkg_name) = it.next()?.kind {
          // Expect semicolong
          it.expect(TokenKind::End)?;

          // Add import
          let parts = name.split("/").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
          f.imports.push(Import { path: parts, name: pkg_name });
          return Ok(());
        }

        let curr = it.curr();
        return Err(ParseError::UnexpectedToken(curr.pos, curr.kind));
      }

      // Expect semicolon
      it.back();
      it.expect(TokenKind::End)?;

      // Add import
      let parts = name.split("/").collect::<Vec<&str>>().iter().map(|x| x.to_string()).collect::<Vec<String>>();
      let pkg_name = parts[parts.len() - 1].clone();
      f.imports.push(Import { path: parts, name: pkg_name });
      return Ok(());
    }

    let curr = it.curr();
    return Err(ParseError::UnexpectedToken(curr.pos, curr.kind));
  }

  // Function Statement
  if next.kind == TokenKind::Ident("fn".to_string()) {
    if let TokenKind::Ident(fn_name) = it.next()?.kind {
      // Parse params
      it.expect(TokenKind::LParen)?;
      let mut params: Vec<Param> = Vec::new();
      if let TokenKind::RParen = it.next()?.kind {} else { // no params
        it.back(); 
        loop {
          // Get param name
          if let TokenKind::Ident(name) = it.next()?.kind {
            let typ = parse_expr(it)?;
            if let TokenKind::RParen = it.next()?.kind { // end of params
              break
            }
            it.back();
            it.expect(TokenKind::Comma)?;
            params.push(Param { name, typ })
          } else {
            let curr = it.curr();
            return Err(ParseError::UnexpectedToken(curr.pos, curr.kind));
          }
        }
      }

      // Parse body
      let ret = match it.expect(TokenKind::LBrack) {
        Ok(_) => None,
        Err(_) => Some(parse_expr(it)?),
      };
      let mut body: Vec<Stmt> = Vec::new();
      loop {
        let stm = parse_stmt(it)?;
        body.push(stm);
        if let TokenKind::RBrack = it.next()?.kind {
          break;
        }
        it.back();
      }

      let func = Function {
        name: fn_name,
        params,
        ret,
        body
      };
      f.functions.insert(func.name.clone(), func);
      return Ok(());
    }

    let curr = it.curr();
    return Err(ParseError::UnexpectedToken(curr.pos, curr.kind));
  }

  Err(ParseError::UnexpectedToken(next.pos, next.kind))
}

fn parse_expr(it: &mut Tokens) -> Result<Expr, ParseError> {
  match it.next()?.kind {
    TokenKind::Ident(val) => {
      match it.next()?.kind {
        // TODO: Some things here

        _ => {
          it.back();
          Ok(Expr{pos: it.curr().pos, kind: ExprKind::Ident(val)})
        }
      }
    }
    _ => Err(ParseError::UnexpectedToken(it.curr().pos, it.curr().kind))
  }
}

// TODO
fn parse_stmt(it: &mut Tokens) -> Result<Stmt, ParseError> {
  let next = it.next()?;
  Ok(Stmt{pos: next.pos, kind: StmtKind::TODO})
}
