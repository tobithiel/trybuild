struct Atype<T, K>(T, K);
struct Btype<T, K>(T, K);
struct Ctype<T, K>(T, K);

fn main() {
    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok("")
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))))))))));
    //~^^^^^ ERROR E0308

    let _ = Some(Ok(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
        Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
            Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(Some(
                Some(Some(Some(Some(Some(Some(Some(Some(Some("")))))))))
            )))))))))))))))))
        ))))))))))))))))))
    ))))))))))))))))) == Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))));
    //~^^^^^ ERROR E0308

    let x: Atype<
      Btype<
        Ctype<
          Atype<
            Btype<
              Ctype<
                Atype<
                  Btype<
                    Ctype<i32, i32>,
                    i32
                  >,
                  i32
                >,
                i32
              >,
              i32
            >,
            i32
          >,
          i32
        >,
        i32
      >,
      i32
    > = ();
    //~^ ERROR E0308

    let _: () = Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
        Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(Ok(
            Ok(Ok(Ok(Ok(Ok(Ok(Ok("")))))))
        ))))))))))))))))))))))))))))))
    ))))))))))))))))))))))));
    //~^^^^^ ERROR E0308
}
