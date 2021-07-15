use criterion::{black_box, criterion_group, criterion_main, Criterion};
use spaceform::base::{Matrix, Vector};

fn component_arithmetic(c: &mut Criterion)
{
	c.bench_function("1000 adds", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(vec + vec);
			}
		})
	});

	c.bench_function("1000 subs", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(vec - vec);
			}
		})
	});

	c.bench_function("1000 muls", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(vec * vec);
			}
		})
	});

	c.bench_function("1000 divs", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(vec / vec);
			}
		})
	});
}

fn misc(c: &mut Criterion)
{
	c.bench_function("1000 abs", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				black_box(vec.abs());
			}
		})
	});

	c.bench_function("1000 hsums", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				black_box(vec.hsum());
			}
		})
	});

	c.bench_function("1000 mins", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec1 = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				let vec2 = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(Vector::min(vec1, vec2));
			}
		})
	});

	c.bench_function("1000 maxes", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec1 = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				let vec2 = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(Vector::max(vec1, vec2));
			}
		})
	});
}

fn products(c: &mut Criterion)
{
	c.bench_function("1000 dots", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec1 = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				let vec2 = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(Vector::dot(vec1, vec2));
			}
		})
	});

	c.bench_function("1000 crosses", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec1 = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				let vec2 = black_box(Vector::new(1f32, 2f32, 3f32, 4f32));
				black_box(Vector::cross(vec1, vec2));
			}
		})
	});
}

fn mul(c: &mut Criterion)
{
	c.bench_function("1000 matrix muls", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let mat = black_box(Matrix::rows([
					[1f32, 2f32, 3f32, 4f32],
					[5f32, 6f32, 7f32, 8f32],
					[9f32, 10f32, 11f32, 12f32],
					[13f32, 14f32, 15f32, 16f32],
				]));
				black_box(mat * mat);
			}
		})
	});

	c.bench_function("1000 vector matrix muls", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let vec = black_box(Vector::new(-1f32, 2f32, -3f32, 4f32));
				let mat = black_box(Matrix::rows([
					[1f32, 2f32, 3f32, 4f32],
					[5f32, 6f32, 7f32, 8f32],
					[9f32, 10f32, 11f32, 12f32],
					[13f32, 14f32, 15f32, 16f32],
				]));
				black_box(vec * mat);
			}
		})
	});
}

fn others(c: &mut Criterion)
{
	c.bench_function("1000 transposes", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let mat = black_box(Matrix::rows([
					[1f32, 2f32, 3f32, 4f32],
					[5f32, 6f32, 7f32, 8f32],
					[9f32, 10f32, 11f32, 12f32],
					[13f32, 14f32, 15f32, 16f32],
				]));
				black_box(mat.transpose());
			}
		})
	});

	c.bench_function("1000 inverses", |b| {
		b.iter(|| {
			for _ in 0..1000
			{
				let mat = black_box(Matrix::rows([
					[1f32, 2f32, 3f32, 4f32],
					[5f32, 6f32, 7f32, 8f32],
					[9f32, 10f32, 11f32, 12f32],
					[13f32, 14f32, 15f32, 16f32],
				]));
				black_box(mat.inverse());
			}
		})
	});
}

criterion_group!(vector, component_arithmetic, misc, products);
criterion_group!(matrix, mul, others);
criterion_main!(vector, matrix);
