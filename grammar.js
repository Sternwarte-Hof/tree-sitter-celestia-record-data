module.exports = grammar({
  name: 'celestia_record_data',

  extras: $ => [
    /\s/,
    $.comment,
  ],

  conflicts: $ => [
    [$.boundary_record],
  ],

  rules: {
    source_file: $ => repeat($._record_entry),

    comment: $ => token(seq('#', /.*/)),

    _record_entry: $ => choice(
      $.xyzv_record,
      $.orientation_record,
      $.xyz_record,
      $.boundary_record,
      $.star_name_record,
    ),

    xyzv_record: $ => prec(5, seq(
      field('time', $.number),
      field('x', $.number),
      field('y', $.number),
      field('z', $.number),
      field('vx', $.number),
      field('vy', $.number),
      field('vz', $.number),
    )),

    orientation_record: $ => prec(4, seq(
      field('time', $.number),
      field('qw', $.number),
      field('qx', $.number),
      field('qy', $.number),
      field('qz', $.number),
    )),

    xyz_record: $ => prec(3, seq(
      field('time', $.number),
      field('x', $.number),
      field('y', $.number),
      field('z', $.number),
    )),

    boundary_record: $ => prec(2, seq(
      field('ra', $.number),
      field('dec', $.number),
      field('code', $.identifier),
      optional(field('marker', $.number)),
    )),

    star_name_record: $ => prec(1, seq(
      field('catalog_number', $.number),
      ':',
      field('name', $.name_segment),
      repeat(seq(
        ':',
        field('alias', $.name_segment),
      )),
    )),

    number: $ => token(seq(
      optional('-'),
      /\d+/,
      optional(seq('.', /\d+/)),
      optional(seq(/[eE]/, optional(/[+-]/), /\d+/))
    )),

    identifier: $ => /[a-zA-Z_][a-zA-Z0-9_-]*/,

    name_segment: $ => /[^:\r\n]+/,
  }
});
