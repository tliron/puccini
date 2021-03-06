// This file was auto-generated from a YAML file

package v1_0

func init() {
	Profile["/hot/1.0/js/constraints/length.js"] = `

exports.validate = function(v, limits) {
	if (arguments.length !== 2)
		throw 'must have 1 argument';
	if ((limits.min === undefined) && (limits.max === undefined))
		throw 'must provide "min" and/or "max"';
	if (v.$string !== undefined)
		v = v.$string;
	if (limits.min !== undefined)
		if (v.length < limits.min)
			return false;
	if (limits.max !== undefined)
		if (v.length > limits.max)
			return false;
	return true;
};
`
}
