use super::*;

pub fn parse_type_expr(it: &mut Tokens) -> Result<TypeExpr, ParseError> {
  match it.next()?.kind {
    TokenKind::Ident(v) => {
      let startpos = it.curr().pos;
      let next = it.next()?;
      if let TokenKind::Selector = next.kind {
        let rhs = parse_type_expr(it)?;
        return Ok(TypeExpr{pos: startpos.extend(rhs.pos.clone()), kind: TypeExprKind::SelectorExpr(v, Box::new(rhs))});
      }
      it.back();
      Ok(TypeExpr{pos: startpos, kind: TypeExprKind::Ident(v)})
    }
    _ => {
      let curr = it.curr();
      Err(ParseError::UnexpectedToken(curr.pos, curr.kind))
    }
  }
}

pub fn parse_expr(it: &mut Tokens) -> Result<Expr, ParseError> {
  // TODO: LR parser
  todo!()
}