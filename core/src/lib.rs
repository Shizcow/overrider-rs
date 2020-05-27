pub fn get_priority(attrs: &Vec<syn::Attribute>) -> Option<i32> {
    for attr in attrs {
	if attr.path.segments[0].ident.to_string() == "override_default" {
	    if let Ok(syn::Expr::Paren(expr)) = syn::parse2::<syn::Expr>(attr.tokens.clone()) {
		if let syn::Expr::Assign(assign) = *expr.expr {
		    if let syn::Expr::Path(left) = *assign.left {
			if left.path.segments.len() == 1
			    && left.path.segments[0].ident.to_string() == "priority" {
				if let syn::Expr::Lit(lit) = *assign.right {
				    if let syn::Lit::Int(i) = lit.lit {
					if let Ok(priority) = i.base10_parse::<i32>() {
					    return Some(priority);
					} else {
					    panic!("Invalid positive integer rvalue in macro invocation");
					}
				    } else {
					panic!("Expected integer rvalue in macro invocation");
				    }
				} else {
				    panic!("Expected rvalue literal in macro invocation");
				}
			    } else {
				panic!("Invalid lvalue in macro invocation");
			    }
		    } else {
			panic!("Unparsable lvalue in macro invocation");
		    }
		} else {
		    panic!("Invalid expression in macro invocation");
		}
	    } else { // might be default
		if attr.tokens.is_empty() {
		    return Some(1);
		} else {
		    panic!("Invalid macro invocation");
		}
	    }
	} else if attr.path.segments[0].ident.to_string() == "default" {
	    if attr.tokens.is_empty() {
		return Some(0);
	    } else {
		panic!("Unexpected arguement in macro invocation");
	    }
	}
    }
    None
}
