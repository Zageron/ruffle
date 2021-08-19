//! `flash.geom.Rectangle` builtin/prototype

use crate::avm1::AvmString;
use crate::avm2::class::{Class, ClassAttributes};
use crate::avm2::globals::flash::geom::point::create_point;
use crate::avm2::method::{Method, NativeMethodImpl};
use crate::avm2::{Activation, Error, Namespace, Object, QName, TObject, Value};
use gc_arena::{GcCell, MutationContext};

pub fn create_rectangle<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    properties: (f64, f64, f64, f64),
) -> Result<Value<'gc>, Error> {
    let rectangle_class = activation.context.avm2.classes().rectangle;

    let args = [
        Value::Number(properties.0),
        Value::Number(properties.1),
        Value::Number(properties.2),
        Value::Number(properties.3),
    ];
    let new_rectangle = rectangle_class.construct(activation, &args)?;

    Ok(new_rectangle.into())
}

macro_rules! get_prop {
    ($this:expr, $activation:expr, $name: expr) => {
        $this
            .get_property($this, &QName::new(Namespace::public(), $name), $activation)?
            .coerce_to_number($activation)
    };
}

macro_rules! set_prop {
    ($this:expr, $activation:expr, $name: expr, $value: expr) => {
        $this.set_property(
            $this,
            &QName::new(Namespace::public(), $name),
            $value.into(),
            $activation,
        )
    };
}

/// Implements `flash.geom.Rectangle`'s instance constructor.
pub fn instance_init<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    let _ = set_to(activation, this, args)?;
    Ok(Value::Undefined)
}

/// Implements `flash.geom.Rectangle`'s class initializer.
pub fn class_init<'gc>(
    _activation: &mut Activation<'_, 'gc, '_>,
    _this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    Ok(Value::Undefined)
}

/// Implement `top`'s getter
pub fn top<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let top = get_prop!(this, activation, "y")?;

        return Ok(top.into());
    }

    Ok(Value::Undefined)
}

/// Implement `top`'s setter
pub fn set_top<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let top = args
            .get(0)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        let height = get_prop!(this, activation, "height")?;

        let y = get_prop!(this, activation, "y")?;

        let height = height + y - top;

        set_prop!(this, activation, "y", top)?;
        set_prop!(this, activation, "height", height)?;
    }

    Ok(Value::Undefined)
}

/// Implement `bottom`'s getter
pub fn bottom<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let y = get_prop!(this, activation, "y")?;
        let height = get_prop!(this, activation, "height")?;

        let bottom = y + height;

        return Ok(bottom.into());
    }

    Ok(Value::Undefined)
}

/// Implement `bottom`'s setter
pub fn set_bottom<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let bottom = args
            .get(0)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        let y = get_prop!(this, activation, "y")?;

        let height = bottom - y;

        set_prop!(this, activation, "height", height)?;
    }

    Ok(Value::Undefined)
}

/// Implement `left`'s getter
pub fn left<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let left = get_prop!(this, activation, "x")?;

        return Ok(left.into());
    }

    Ok(Value::Undefined)
}

/// Implement `left`'s setter
pub fn set_left<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let left = args
            .get(0)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        let width = get_prop!(this, activation, "width")?;

        let x = get_prop!(this, activation, "x")?;

        let width = width + x - left;

        set_prop!(this, activation, "x", left)?;
        set_prop!(this, activation, "width", width)?;
    }

    Ok(Value::Undefined)
}

/// Implement `right`'s getter
pub fn right<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let x = get_prop!(this, activation, "x")?;
        let width = get_prop!(this, activation, "width")?;

        let right = x + width;

        return Ok(right.into());
    }

    Ok(Value::Undefined)
}

/// Implement `right`'s setter
pub fn set_right<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let right = args
            .get(0)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        let x = get_prop!(this, activation, "x")?;

        let width = right - x;

        set_prop!(this, activation, "width", width)?;
    }

    Ok(Value::Undefined)
}

/// Implement `bottomRight`'s getter
pub fn bottom_right<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let y = get_prop!(this, activation, "y")?;
        let height = get_prop!(this, activation, "height")?;

        let bottom = y + height;

        let x = get_prop!(this, activation, "x")?;
        let width = get_prop!(this, activation, "width")?;

        let right = x + width;

        return create_point(activation, (right, bottom));
    }

    Ok(Value::Undefined)
}

/// Implement `bottomRight`'s setter
pub fn set_bottom_right<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        if let Some(point) = args.get(0) {
            let point_obj = point.coerce_to_object(activation)?;
            let right = get_prop!(point_obj, activation, "x")?;
            let bottom = get_prop!(point_obj, activation, "y")?;

            let x = get_prop!(this, activation, "x")?;
            let y = get_prop!(this, activation, "y")?;
            let width = right - x;
            let height = bottom - y;

            set_prop!(this, activation, "width", width)?;
            set_prop!(this, activation, "height", height)?;
        }
    }

    Ok(Value::Undefined)
}

/// Implement `topLeft`'s getter
pub fn top_left<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let x = get_prop!(this, activation, "x")?;
        let y = get_prop!(this, activation, "y")?;

        return create_point(activation, (x, y));
    }

    Ok(Value::Undefined)
}

/// Implement `topLeft`'s setter
pub fn set_top_left<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        if let Some(point) = args.get(0) {
            let point_obj = point.coerce_to_object(activation)?;

            let left = get_prop!(point_obj, activation, "x")?;
            let top = get_prop!(point_obj, activation, "y")?;

            let x = get_prop!(this, activation, "x")?;
            let y = get_prop!(this, activation, "y")?;

            let width = get_prop!(this, activation, "width")? - left + x;
            let height = get_prop!(this, activation, "height")? - top + y;

            set_prop!(this, activation, "x", left)?;
            set_prop!(this, activation, "y", top)?;
            set_prop!(this, activation, "width", width)?;
            set_prop!(this, activation, "height", height)?;
        }
    }

    Ok(Value::Undefined)
}

/// Implement `size`'s getter
pub fn size<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let width = get_prop!(this, activation, "width")?;
        let height = get_prop!(this, activation, "height")?;

        return create_point(activation, (width, height));
    }

    Ok(Value::Undefined)
}

/// Implement `size`'s setter
pub fn set_size<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        if let Some(point) = args.get(0) {
            let point_obj = point.coerce_to_object(activation)?;

            let width = get_prop!(point_obj, activation, "x")?;
            let height = get_prop!(point_obj, activation, "y")?;

            set_prop!(this, activation, "width", width)?;
            set_prop!(this, activation, "height", height)?;
        }
    }

    Ok(Value::Undefined)
}

/// Implement `clone`
pub fn clone<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let x = get_prop!(this, activation, "x")?;
        let y = get_prop!(this, activation, "y")?;

        let width = get_prop!(this, activation, "width")?;
        let height = get_prop!(this, activation, "height")?;

        return create_rectangle(activation, (x, y, width, height));
    }

    Ok(Value::Undefined)
}

fn contains_tuple<'gc>(
    this: &Object<'gc>,
    activation: &mut Activation<'_, 'gc, '_>,
    coords: (f64, f64),
) -> Result<bool, Error> {
    let (x, y) = coords;
    let left = get_prop!(*this, activation, "x")?;
    let right = get_prop!(*this, activation, "width")? + left;

    let top = get_prop!(*this, activation, "y")?;
    let bottom = get_prop!(*this, activation, "height")? + top;

    return Ok(left <= x && x < right && top <= y && y < bottom);
}

/// Implement `contains`
pub fn contains<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let x = args.get(0).unwrap().coerce_to_number(activation)?;
        let y = args.get(1).unwrap().coerce_to_number(activation)?;

        return Ok(contains_tuple(&this, activation, (x, y))?.into());
    }

    Ok(Value::Undefined)
}

/// Implement `containsPoint`
pub fn contains_point<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(point) = args.get(0) {
            let point = point.coerce_to_object(activation)?;
            let x = get_prop!(point, activation, "x")?;
            let y = get_prop!(point, activation, "y")?;

            return Ok(contains_tuple(&this, activation, (x, y))?.into());
        }
    }

    Ok(Value::Undefined)
}

/// Implement `containsRect`
pub fn contains_rect<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(rect) = args.get(0) {
            let rect = rect.coerce_to_object(activation)?;
            let x = get_prop!(rect, activation, "x")?;
            let y = get_prop!(rect, activation, "y")?;
            let right = get_prop!(rect, activation, "width")? + x;
            let bottom = get_prop!(rect, activation, "height")? + y;

            return Ok((contains_tuple(&this, activation, (x, y))?
                && contains_tuple(&this, activation, (right, bottom))?)
            .into());
        }
    }

    Ok(Value::Undefined)
}

/// Implement `copyFrom`
pub fn copy_from<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(rect) = args.get(0) {
        let rect = rect.coerce_to_object(activation)?;
        let x = rect.get_property(rect, &QName::new(Namespace::public(), "x"), activation)?;
        let y = rect.get_property(rect, &QName::new(Namespace::public(), "y"), activation)?;
        let width =
            rect.get_property(rect, &QName::new(Namespace::public(), "width"), activation)?;
        let height =
            rect.get_property(rect, &QName::new(Namespace::public(), "height"), activation)?;

        set_to(activation, this, &[x, y, width, height])?;
    }

    Ok(Value::Undefined)
}

/// Implement `equals`
#[allow(clippy::float_cmp)]
pub fn equals<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        if let Some(other) = args.get(0) {
            let other = other.coerce_to_object(activation)?;

            let x = get_prop!(this, activation, "x")? == get_prop!(other, activation, "x")?;
            let y = get_prop!(this, activation, "y")? == get_prop!(other, activation, "y")?;
            let width =
                get_prop!(this, activation, "width")? == get_prop!(other, activation, "width")?;
            let height =
                get_prop!(this, activation, "height")? == get_prop!(other, activation, "height")?;

            return Ok((x && y && width && height).into());
        }
    }

    Ok(Value::Undefined)
}

/// Implements `setTo`
pub fn set_to<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(mut this) = this {
        let x = args
            .get(0)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;
        let y = args
            .get(1)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        let width = args
            .get(2)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;
        let height = args
            .get(3)
            .unwrap_or(&0.into())
            .coerce_to_number(activation)?;

        set_prop!(this, activation, "x", x)?;
        set_prop!(this, activation, "y", y)?;
        set_prop!(this, activation, "width", width)?;
        set_prop!(this, activation, "height", height)?;
    }

    Ok(Value::Undefined)
}

/// Implements `toString`
pub fn to_string<'gc>(
    activation: &mut Activation<'_, 'gc, '_>,
    this: Option<Object<'gc>>,
    _args: &[Value<'gc>],
) -> Result<Value<'gc>, Error> {
    if let Some(this) = this {
        let x = this
            .get_property(this, &QName::new(Namespace::public(), "x"), activation)?
            .coerce_to_string(activation)?;
        let y = this
            .get_property(this, &QName::new(Namespace::public(), "y"), activation)?
            .coerce_to_string(activation)?;
        let width = this
            .get_property(this, &QName::new(Namespace::public(), "width"), activation)?
            .coerce_to_string(activation)?;
        let height = this
            .get_property(this, &QName::new(Namespace::public(), "height"), activation)?
            .coerce_to_string(activation)?;

        return Ok(AvmString::new(
            activation.context.gc_context,
            format!("(x={}, y={}, w={}, h={})", x, y, width, height),
        )
        .into());
    }

    Ok(Value::Undefined)
}

/// Construct `Rectangle`'s class.
pub fn create_class<'gc>(mc: MutationContext<'gc, '_>) -> GcCell<'gc, Class<'gc>> {
    let class = Class::new(
        QName::new(Namespace::package("flash.geom"), "Rectangle"),
        Some(QName::new(Namespace::public(), "Object").into()),
        Method::from_builtin(instance_init, "<Rectangle instance initializer>", mc),
        Method::from_builtin(class_init, "<Rectangle class initializer>", mc),
        mc,
    );

    let mut write = class.write(mc);
    write.set_attributes(ClassAttributes::SEALED);

    const PUBLIC_INSTANCE_PROPERTIES: &[(
        &str,
        Option<NativeMethodImpl>,
        Option<NativeMethodImpl>,
    )] = &[
        ("top", Some(top), Some(set_top)),
        ("bottom", Some(bottom), Some(set_bottom)),
        ("left", Some(left), Some(set_left)),
        ("right", Some(right), Some(set_right)),
        ("bottomRight", Some(bottom_right), Some(set_bottom_right)),
        ("topLeft", Some(top_left), Some(set_top_left)),
        ("size", Some(size), Some(set_size)),
    ];
    write.define_public_builtin_instance_properties(mc, PUBLIC_INSTANCE_PROPERTIES);

    const PUBLIC_INSTANCE_METHODS: &[(&str, NativeMethodImpl)] = &[
        ("contains", contains),
        ("containsPoint", contains_point),
        ("containsRect", contains_rect),
        ("copyFrom", copy_from),
        ("equals", equals),
        ("clone", clone),
        ("setTo", set_to),
        ("toString", to_string),
    ];
    write.define_public_builtin_instance_methods(mc, PUBLIC_INSTANCE_METHODS);
    class
}