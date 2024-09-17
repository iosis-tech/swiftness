from starkware.cairo.lang.compiler.parser import parse_file
from starkware.cairo.lang.compiler.ast.code_elements import *
from starkware.cairo.lang.compiler.ast.bool_expr import *
from starkware.cairo.lang.compiler.ast.expr import *
from starkware.cairo.lang.compiler.ast.expr_func_call import *
import requests

DYNAMIC_INDEXABLE = {'ec_op_doubled_points_y_column', 'pedersen_hash0_ec_subset_sum_selector_column', 'add_mod_carry1_bit_column', 'keccak_keccak_rotated_parity1_column', 'mem_pool_addr_column', 'ec_op_ec_subset_sum_bit_unpacking_prod_ones192_column', 'ecdsa_signature0_key_points_x_column', 'ecdsa_signature0_exponentiate_key_selector_column', 'poseidon_poseidon_partial_rounds_state1_squared_column', 'ec_op_doubled_points_x_column', 'ecdsa_signature0_add_results_slope_column', 'cpu_update_registers_update_pc_tmp1_column', 'ec_op_ec_subset_sum_x_diff_inv_column', 'pedersen_hash0_ec_subset_sum_partial_sum_x_column', 'range_check16_sorted_column', 'ecdsa_signature0_extract_r_inv_column', 'memory_multi_column_perm_perm_cum_prod0_column', 'keccak_keccak_rotated_parity2_column', 'ecdsa_signature0_doubling_slope_column', 'poseidon_poseidon_full_rounds_state2_squared_column', 'ec_op_doubling_slope_column', 'diluted_check_permuted_values_column', 'cpu_decode_opcode_range_check_column_column', 'mem_pool_value_column', 'ecdsa_signature0_exponentiate_key_slope_column', 'keccak_keccak_parse_to_diluted_reshaped_intermediate_column', 'pedersen_hash0_ec_subset_sum_slope_column', 'pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones196_column', 'ecdsa_signature0_exponentiate_generator_selector_column', 'ecdsa_signature0_add_results_inv_column', 'ecdsa_signature0_key_points_y_column', 'cpu_operands_ops_mul_column', 'ecdsa_signature0_exponentiate_key_partial_sum_y_column', 'poseidon_poseidon_full_rounds_state1_squared_column', 'ecdsa_signature0_exponentiate_generator_partial_sum_x_column', 'cpu_registers_fp_column', 'poseidon_poseidon_full_rounds_state0_squared_column', 'pedersen_hash0_ec_subset_sum_partial_sum_y_column', 'keccak_keccak_parse_to_diluted_cumulative_sum_column', 'diluted_check_permutation_cum_prod0_column', 'keccak_keccak_rotated_parity4_column', 'ec_op_ec_subset_sum_bit_unpacking_prod_ones196_column', 'ec_op_ec_subset_sum_partial_sum_x_column', 'memory_sorted_addr_column', 'keccak_keccak_rotated_parity0_column', 'diluted_pool_column', 'add_mod_carry2_sign_column', 'poseidon_poseidon_partial_rounds_state0_column', 'poseidon_poseidon_full_rounds_state1_column', 'memory_sorted_value_column', 'ecdsa_signature0_exponentiate_generator_x_diff_inv_column', 'add_mod_carry1_sign_column', 'cpu_update_registers_update_pc_tmp0_column', 'add_mod_carry2_bit_column', 'poseidon_poseidon_partial_rounds_state0_squared_column', 'ecdsa_signature0_z_inv_column', 'ec_op_ec_subset_sum_selector_column', 'pedersen_hash0_ec_subset_sum_bit_unpacking_prod_ones192_column', 'keccak_keccak_rotated_parity3_column', 'ec_op_ec_subset_sum_slope_column', 'poseidon_poseidon_full_rounds_state2_column', 'ecdsa_signature0_exponentiate_key_partial_sum_x_column', 'ec_op_ec_subset_sum_partial_sum_y_column', 'ecdsa_signature0_exponentiate_generator_partial_sum_y_column', 'range_check16_pool_column', 'keccak_keccak_parse_to_diluted_final_reshaped_input_column', 'add_mod_sub_p_bit_column', 'poseidon_poseidon_partial_rounds_state1_column', 'poseidon_poseidon_full_rounds_state0_column', 'add_mod_carry3_sign_column', 'cpu_registers_ap_column', 'diluted_check_cumulative_value_column', 'ecdsa_signature0_exponentiate_generator_slope_column', 'ecdsa_signature0_q_x_squared_column', 'ecdsa_signature0_extract_r_slope_column', 'ecdsa_signature0_exponentiate_key_x_diff_inv_column', 'ecdsa_signature0_r_w_inv_column', 'add_mod_carry3_bit_column', 'range_check16_perm_cum_prod0_column', 'cpu_operands_res_column', 'num_columns_first', 'num_columns_second'}

def rename_var(name: str) -> str:
    return name.replace("__", "_")


def parse(node: AstNode, comment: str = "") -> str:
    match node:
        case CodeBlock(code_elements=code_elements):
            acc = ""
            for ce in code_elements:
                acc += parse(ce)
            return acc

        case CodeElementAllocLocals():  # alloc_locals
            return ""

        case (
            CodeElementLocalVariable(  # local x
                typed_identifier=TypedIdentifier(identifier=ExprIdentifier(name=name)),
                expr=expr,
            )
            | CodeElementUnpackBinding(  # let (local x)
                unpacking_list=IdentifierList(
                    identifiers=[TypedIdentifier(identifier=ExprIdentifier(name=name))]
                ),
                rvalue=expr,
            )
            | CodeElementTemporaryVariable(  # tempvar x
                typed_identifier=TypedIdentifier(identifier=ExprIdentifier(name=name)),
                expr=expr,
            )
        ):
            com = "" if comment is None else (" //" + comment)
            if name == "range_check_ptr": return ""

            if expr is None:
                expr = ExprConst(val=0, format_str='0')
                return f"let mut {rename_var(name)} = {parse(expr)};{com}\n\t"
            
            if isinstance(expr, ExprIdentifier) and expr.name.startswith("dynamic_params."):
                suffix = rename_var(expr.name.split("dynamic_params.")[1])
                if suffix in DYNAMIC_INDEXABLE:
                    return ""
                return f"let {rename_var(name)} = felt!({parse(expr)});{com}\n\t"
            
            return f"let {rename_var(name)} = {parse(expr)};{com}\n\t"

        case RvalueFuncCall(  # safe_div(x, y)
            func_ident=ExprIdentifier(name="safe_div"), arguments=ArgList(args=[lv, rv])
        ):
            return f"{parse(lv)}.floor_div(&felt_nonzero!({parse(rv)}))"

        case RvalueFuncCall(  # pow(x, y)
            func_ident=ExprIdentifier(name="pow"), arguments=ArgList(args=[lv, rv])
        ):
            return f"{parse(lv)}.pow_felt(&({parse(rv)}))"

        case RvalueFuncCall(  # safe_mult(x,y)
            func_ident=ExprIdentifier(name="safe_mult"),
            arguments=ArgList(args=[lv, rv]),
        ):
            # TODO: should this be safe_mult?
            return f"{parse(lv)} * {parse(rv)}"

        case RvalueFuncCall(  # f(x, y, ...)
            func_ident=ExprIdentifier(name=name),
            arguments=ArgList(args=args),
        ):

            def remove_parenthesis(arg):
                match arg:
                    case ExprAssignment(expr=ExprParentheses(val=val)):
                        return val
                return arg

            return (
                f"{name}({', '.join([parse(remove_parenthesis(arg)) for arg in args])})"
            )

        case ExprOperator(a=a, b=b, op="/"):
            return f"{parse(a)}.field_div(&felt_nonzero!({parse(b)}))"

        case ExprOperator(a=a, b=b, op=op):
            return f"{parse(a)} {op} {parse(b)}"

        case ExprSubscript(  # x[0]
            expr=ExprIdentifier(name=name), offset=ExprConst(val=val)
        ):
            return f"{name}[{val}]"

        case ExprSubscript(  # x[CONST_VAR]
            expr=ExprIdentifier(name=name), offset=offset
        ):
            if str(parse(offset)) in DYNAMIC_INDEXABLE:
                return f"{name}[dynamic_params.{parse(offset)}]"
            return f"{name}[{parse(offset)}]"

        case CodeElementStaticAssert(a=a, b=b):  # static assert x == y
            return f"assert({parse(a)} == {parse(b)}, 'Assert failed');\n\t"

        case CodeElementReturn(  # return (res=x)
            expr=ExprTuple(
                members=ArgList(
                    args=[
                        ExprAssignment(
                            identifier=ExprIdentifier(name="res"),
                            expr=ExprIdentifier(name=var),
                        )
                    ]
                )
            )
        ):
            return f"{var}\n"

        case ExprParentheses(val=val):  # (x)
            return f"({parse(val)})"

        case ExprIdentifier(name=name):  # x
            return rename_var(name)

        case ExprConst(format_str=format_str):
            return f"FELT_{format_str}"

        case ExprAssignment(expr=expr):
            return parse(expr)

        case ExprFuncCall(rvalue=rvalue):
            return parse(rvalue)

        case CommentedCodeElement(code_elm=code_elm, comment=comment):
            return parse(code_elm, comment)

        case CodeElementEmptyLine():
            if comment is None:
                return "\n\t"
            return "//" + comment + "\n\t"

        case ExprCast(expr=expr):  # cast()
            return ""

        case CodeElementIf(
            condition=condition,
            main_code_block=main_code_block,
        ):
            return f"if {parse(condition)} {{{parse(main_code_block)}}}\n\t"
        
        case BoolEqExpr(
            a=a,
            b=b,
            eq=eq,
        ):
            return f"{parse(a)} {"==" if eq else "!="} {parse(b)}"
        
        case CodeElementCompoundAssertEq(
            a=a,
            b=b,
        ):
            return f"{parse(a)} = {parse(b)};\n\t"
        
        case ExprCast(expr=expr):  # cast()
            return ""

    print(node.__class__.__name__, "not implemented")
    print(node, "\n")
    return ""


def handle_github_file(url, output_file):
    global array_read_offset
    response = requests.get(url)
    if response.status_code != 200:
        raise Exception(f"Failed to fetch {url}")

    ast = parse_file(response.text, filename="autogenerated.cairo")

    global constants
    constants = {}

    functions_result = {}
    for commented_code_element in ast.code_block.code_elements:
        match commented_code_element.code_elm:
            case CodeElementFunction(
                element_type="func",
                identifier=ExprIdentifier(name=name),
                code_block=code_block,
            ):
                functions_result[name] = name + " {" + parse(code_block) + "}\n"

    with open(output_file, "w") as f:
        f.write("\n".join(functions_result.values()))


def main():
    layouts = ["dynamic"]

    for layout in layouts:
        handle_github_file(
            f"https://raw.githubusercontent.com/starkware-libs/cairo-lang/master/src/starkware/cairo/stark_verifier/air/layouts/{layout}/autogenerated.cairo",
            f"output/{layout}.rs",
        )


if __name__ == "__main__":
    main()
