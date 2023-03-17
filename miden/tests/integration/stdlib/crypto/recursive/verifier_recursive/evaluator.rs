use winter_air::{Air, AuxTraceRandElements, ConstraintCompositionCoefficients, EvaluationFrame};
use winter_utils::collections::Vec;
use winterfell::math::{polynom, FieldElement};

// CONSTRAINT EVALUATION
// ================================================================================================

/// Evaluates constraints for the specified evaluation frame.
pub fn evaluate_constraints<A: Air, E: FieldElement<BaseField = A::BaseField>>(
    air: &A,
    composition_coefficients: ConstraintCompositionCoefficients<E>,
    main_trace_frame: &EvaluationFrame<E>,
    aux_trace_frame: &Option<EvaluationFrame<E>>,
    aux_rand_elements: AuxTraceRandElements<E>,
    x: E,
) -> E {
    // 1 ----- evaluate transition constraints ----------------------------------------------------

    //println!("Eval frame current {:?}", main_trace_frame.current().len());
    //println!("Eval frame next {:?}", main_trace_frame.next().len());
    //println!("composition_coefficients.transition {:?}", composition_coefficients.transition);
    //println!("composition_coefficients.boundry {:?}", composition_coefficients.boundary);
    // initialize a buffer to hold transition constraint evaluations
    let t_constraints = air.get_transition_constraints(&composition_coefficients.transition);

    // compute values of periodic columns at x
    let periodic_values = air
        .get_periodic_column_polys()
        .iter()
        .map(|poly| {
            let num_cycles = air.trace_length() / poly.len();
            let x = x.exp_vartime((num_cycles as u32).into());
            polynom::eval(poly, x)
        })
        .collect::<Vec<_>>();
    println!("number of periodic values {:?}", periodic_values.len());

    // evaluate transition constraints for the main trace segment
    let mut t_evaluations1 = E::zeroed_vector(t_constraints.num_main_constraints());
    air.evaluate_transition(main_trace_frame, &periodic_values, &mut t_evaluations1);
    println!("transition evaluations {:?}", t_evaluations1.len());

    // evaluate transition constraints for auxiliary trace segments (if any)
    let mut t_evaluations2 = E::zeroed_vector(t_constraints.num_aux_constraints());
    if let Some(aux_trace_frame) = aux_trace_frame {
        air.evaluate_aux_transition(
            main_trace_frame,
            aux_trace_frame,
            &periodic_values,
            &aux_rand_elements,
            &mut t_evaluations2,
        );
    }
    println!("transition aux evaluations {:?}", t_evaluations2.len());

    // merge all constraint evaluations into a single value by computing their random linear
    // combination using coefficients drawn from the public coin. this also divides the result
    // by the divisor of transition constraints.
    let mut result = t_constraints.combine_evaluations::<E>(&t_evaluations1, &t_evaluations2, x);

    // 2 ----- evaluate boundary constraints ------------------------------------------------------

    // get boundary constraints grouped by common divisor from the AIR
    let b_constraints =
        air.get_boundary_constraints(&aux_rand_elements, &composition_coefficients.boundary);

    // cache power of x here so that we only re-compute it when degree_adjustment changes
    let mut degree_adjustment = b_constraints.main_constraints()[0].degree_adjustment();
    let mut xp = x.exp_vartime(degree_adjustment.into());

    println!("boundry constraint main {:?}", b_constraints.main_constraints().len());
    println!("boundry constraint aux {:?}", b_constraints.aux_constraints().len());
    println!("degree adjustment is {:?}", degree_adjustment);
    println!("number of comp coef for boundry {:?}", composition_coefficients.boundary.len());
    println!(
        "number of comp coef for transition {:?}",
        composition_coefficients.transition.len()
    );
    // iterate over boundary constraint groups for the main trace segment (each group has a
    // distinct divisor), evaluate constraints in each group and add their combination to the
    // result
    for group in b_constraints.main_constraints().iter() {
        // if adjustment degree hasn't changed, no need to recompute `xp` - so just reuse the
        // previous value; otherwise, compute new `xp`
        if group.degree_adjustment() != degree_adjustment {
            degree_adjustment = group.degree_adjustment();
            xp = x.exp_vartime(degree_adjustment.into());
        }
        println!("degree adjustment is {:?}", degree_adjustment);
        // evaluate all constraints in the group, and add the evaluation to the result
        result += group.evaluate_at(main_trace_frame.current(), x, xp);
    }

    // iterate over boundary constraint groups for auxiliary trace segments (each group has a
    // distinct divisor), evaluate constraints in each group and add their combination to the
    // result
    if let Some(aux_trace_frame) = aux_trace_frame {
        for group in b_constraints.aux_constraints().iter() {
            // if adjustment degree hasn't changed, no need to recompute `xp` - so just reuse the
            // previous value; otherwise, compute new `xp`
            if group.degree_adjustment() != degree_adjustment {
                degree_adjustment = group.degree_adjustment();
                xp = x.exp_vartime(degree_adjustment.into());
            }
            // evaluate all constraints in the group, and add the evaluation to the result
            result += group.evaluate_at(aux_trace_frame.current(), x, xp);
        }
    }

    result
}
