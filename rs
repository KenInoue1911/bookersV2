
Bookモデルのテスト
  バリデーションのテスト
    titleカラム
[32m      空欄でないこと[0m
    bodyカラム
[32m      空欄でないこと[0m
[32m      200文字以下であること: 200文字は〇[0m
[32m      200文字以下であること: 201文字は×[0m
  アソシエーションのテスト
    Userモデルとの関係
[32m      N:1となっている[0m

Userモデルのテスト
  バリデーションのテスト
    nameカラム
[32m      空欄でないこと[0m
[32m      2文字以上であること: 1文字は×[0m
[32m      2文字以上であること: 2文字は〇[0m
[32m      20文字以下であること: 20文字は〇[0m
[32m      20文字以下であること: 21文字は×[0m
[32m      一意性があること[0m
    introductionカラム
[32m      50文字以下であること: 50文字は〇[0m
[32m      50文字以下であること: 51文字は×[0m
  アソシエーションのテスト
    Bookモデルとの関係
[32m      1:Nとなっている[0m

[STEP1] ユーザログイン前のテスト
  トップ画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      Log inリンクが表示される: 左上から5番目のリンクが「Log in」である[0m
[32m      Log inリンクの内容が正しい[0m
[32m      Sign Upリンクが表示される: 左上から6番目のリンクが「Sign Up」である[0m
[32m      Sign Upリンクの内容が正しい[0m
  アバウト画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
  ヘッダーのテスト: ログインしていない場合
    表示内容の確認
[32m      タイトルが表示される[0m
[32m      Homeリンクが表示される: 左上から1番目のリンクが「Home」である[0m
[32m      Aboutリンクが表示される: 左上から2番目のリンクが「About」である[0m
[32m      sign upリンクが表示される: 左上から3番目のリンクが「sign up」である[0m
[32m      loginリンクが表示される: 左上から4番目のリンクが「login」である[0m
    リンクの内容を確認
[32m      Homeを押すと、トップ画面に遷移する[0m
[32m      Aboutを押すと、アバウト画面に遷移する[0m
[32m      sign upを押すと、新規登録画面に遷移する[0m
[32m      loginを押すと、ログイン画面に遷移する[0m
  ユーザ新規登録のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Sign up」と表示される[0m
[32m      nameフォームが表示される[0m
[32m      emailフォームが表示される[0m
[32m      passwordフォームが表示される[0m
[32m      password_confirmationフォームが表示される[0m
[32m      Sign upボタンが表示される[0m
    新規登録成功のテスト
[32m      正しく新規登録される[0m
[32m      新規登録後のリダイレクト先が、新規登録できたユーザの詳細画面になっている[0m
  ユーザログイン
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Log in」と表示される[0m
[32m      nameフォームが表示される[0m
[32m      passwordフォームが表示される[0m
[32m      Log inボタンが表示される[0m
[32m      emailフォームは表示されない[0m
    ログイン成功のテスト
[32m      ログイン後のリダイレクト先が、ログインしたユーザの詳細画面になっている[0m
    ログイン失敗のテスト
[32m      ログインに失敗し、ログイン画面にリダイレクトされる[0m
  ヘッダーのテスト: ログインしている場合
    ヘッダーの表示を確認
[32m      タイトルが表示される[0m
[32m      Homeリンクが表示される: 左上から1番目のリンクが「Home」である[0m
[32m      Usersリンクが表示される: 左上から2番目のリンクが「Users」である[0m
[32m      Booksリンクが表示される: 左上から3番目のリンクが「Books」である[0m
[32m      log outリンクが表示される: 左上から4番目のリンクが「logout」である[0m
  ユーザログアウトのテスト
    ログアウト機能のテスト
[32m      正しくログアウトできている: ログアウト後のリダイレクト先においてAbout画面へのリンクが存在する[0m
[32m      ログアウト後のリダイレクト先が、トップになっている[0m

[STEP2] ユーザログイン後のテスト
  ヘッダーのテスト: ログインしている場合
    リンクの内容を確認: ※logoutは『ユーザログアウトのテスト』でテスト済みになります。
[32m      Homeを押すと、自分のユーザ詳細画面に遷移する[0m
[32m      Usersを押すと、ユーザ一覧画面に遷移する[0m
[32m      Booksを押すと、投稿一覧画面に遷移する[0m
  投稿一覧画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      自分と他人の画像のリンク先が正しい[0m
[32m      自分の投稿と他人の投稿のタイトルのリンク先がそれぞれ正しい[0m
[32m      自分の投稿と他人の投稿のオピニオンが表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      opinionフォームが表示される[0m
[32m      opinionフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
    投稿成功のテスト
[32m      自分の新しい投稿が正しく保存される[0m
[32m      リダイレクト先が、保存できた投稿の詳細画面になっている[0m
  自分の投稿詳細画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[32m      「Book detail」と表示される[0m
[32m      ユーザ画像・名前のリンク先が正しい[0m
[32m      投稿のtitleが表示される[0m
[32m      投稿のopinionが表示される[0m
[32m      投稿の編集リンクが表示される[0m
[32m      投稿の削除リンクが表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      opinionフォームが表示される[0m
[32m      opinionフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
    投稿成功のテスト
[32m      自分の新しい投稿が正しく保存される[0m
    編集リンクのテスト
[32m      編集画面に遷移する[0m
    削除リンクのテスト
[32m      正しく削除される[0m
[32m      リダイレクト先が、投稿一覧画面になっている[0m
  自分の投稿編集画面のテスト
    表示の確認
[32m      URLが正しい[0m
[32m      「Editing Book」と表示される[0m
[32m      title編集フォームが表示される[0m
[32m      opinion編集フォームが表示される[0m
[32m      Update Bookボタンが表示される[0m
[32m      Showリンクが表示される[0m
[32m      Backリンクが表示される[0m
    編集成功のテスト
[32m      titleが正しく更新される[0m
[32m      bodyが正しく更新される[0m
[32m      リダイレクト先が、更新した投稿の詳細画面になっている[0m
  ユーザ一覧画面のテスト
    表示内容の確認
[32m      URLが正しい[0m
[31m      自分と他人の画像が表示される: fallbackの画像がサイドバーの1つ＋一覧(2人)の2つの計3つ存在する (FAILED - 1)[0m
[32m      自分と他人の名前がそれぞれ表示される[0m
[32m      自分と他人のshowリンクがそれぞれ表示される[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      opinionフォームが表示される[0m
[32m      opinionフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
  自分のユーザ詳細画面のテスト
    表示の確認
[32m      URLが正しい[0m
[32m      投稿一覧のユーザ画像のリンク先が正しい[0m
[32m      投稿一覧に自分の投稿のtitleが表示され、リンクが正しい[0m
[32m      投稿一覧に自分の投稿のopinionが表示される[0m
[32m      他人の投稿は表示されない[0m
    サイドバーの確認
[32m      自分の名前と紹介文が表示される[0m
[32m      自分のユーザ編集画面へのリンクが存在する[0m
[32m      「New book」と表示される[0m
[32m      titleフォームが表示される[0m
[32m      titleフォームに値が入っていない[0m
[32m      opinionフォームが表示される[0m
[32m      opinionフォームに値が入っていない[0m
[32m      Create Bookボタンが表示される[0m
  自分のユーザ情報編集画面のテスト
    表示の確認
[32m      URLが正しい[0m
[31m      名前編集フォームに自分の名前が表示される (FAILED - 2)[0m
[32m      画像編集フォームが表示される[0m
[31m      自己紹介編集フォームに自分の自己紹介文が表示される (FAILED - 3)[0m
[32m      Update Userボタンが表示される[0m
    更新成功のテスト
[32m      nameが正しく更新される[0m
[32m      introductionが正しく更新される[0m
[32m      リダイレクト先が、自分のユーザ詳細画面になっている[0m

[STEP3] 仕上げのテスト
  サクセスメッセージのテスト
[32m    ユーザ新規登録成功時[0m
[32m    ユーザログイン成功時[0m
[32m    ユーザログアウト成功時[0m
[31m    ユーザのプロフィール情報更新成功時 (FAILED - 4)[0m
[32m    投稿データの新規投稿成功時: 投稿一覧画面から行います。[0m
[32m    投稿データの更新成功時[0m
  処理失敗時のテスト
    ユーザ新規登録失敗: nameを1文字にする
[32m      新規登録されない[0m
[32m      新規登録画面を表示しており、フォームの内容が正しい[0m
[32m      バリデーションエラーが表示される[0m
    ユーザのプロフィール情報編集失敗: nameを1文字にする
[32m      更新されない[0m
[31m      ユーザ編集画面を表示しており、フォームの内容が正しい (FAILED - 5)[0m
[32m      バリデーションエラーが表示される[0m
    投稿データの新規投稿失敗: 投稿一覧画面から行い、titleを空にする
[32m      投稿が保存されない[0m
[32m      投稿一覧画面を表示している[0m
[32m      新規投稿フォームの内容が正しい[0m
[32m      バリデーションエラーが表示される[0m
    投稿データの更新失敗: titleを空にする
[32m      投稿が更新されない[0m
[32m      投稿編集画面を表示しており、フォームの内容が正しい[0m
[32m      エラーメッセージが表示される[0m
  ログインしていない場合のアクセス制限のテスト: アクセスできず、ログイン画面に遷移する
[32m    ユーザ一覧画面[0m
[32m    ユーザ詳細画面[0m
[32m    ユーザ情報編集画面[0m
[32m    投稿一覧画面[0m
[32m    投稿詳細画面[0m
[32m    投稿編集画面[0m
  他人の画面のテスト
    他人の投稿詳細画面のテスト
      表示内容の確認
[32m        URLが正しい[0m
[32m        「Book detail」と表示される[0m
[32m        ユーザ画像・名前のリンク先が正しい[0m
[32m        投稿のtitleが表示される[0m
[32m        投稿のopinionが表示される[0m
[32m        投稿の編集リンクが表示されない[0m
[32m        投稿の削除リンクが表示されない[0m
      サイドバーの確認
[32m        他人の名前と紹介文が表示される[0m
[31m        他人のユーザ編集画面へのリンクが存在する (FAILED - 6)[0m
[32m        自分の名前と紹介文は表示されない[0m
[32m        自分のユーザ編集画面へのリンクは存在しない[0m
    他人の投稿編集画面
[32m      遷移できず、投稿一覧画面にリダイレクトされる[0m
    他人のユーザ詳細画面のテスト
      表示の確認
[32m        URLが正しい[0m
[32m        投稿一覧のユーザ画像のリンク先が正しい[0m
[32m        投稿一覧に他人の投稿のtitleが表示され、リンクが正しい[0m
[32m        投稿一覧に他人の投稿のopinionが表示される[0m
[32m        自分の投稿は表示されない[0m
      サイドバーの確認
[32m        他人の名前と紹介文が表示される[0m
[31m        他人のユーザ編集画面へのリンクが存在する (FAILED - 7)[0m
[32m        自分の名前と紹介文は表示されない[0m
[32m        自分のユーザ編集画面へのリンクは存在しない[0m
    他人のユーザ情報編集画面
[32m      遷移できず、自分のユーザ詳細画面にリダイレクトされる[0m
  グリッドシステムのテスト: container, row, col-md-〇を正しく使えている
[32m    ユーザ一覧画面[0m
[32m    ユーザ詳細画面[0m
[32m    投稿一覧画面[0m
[32m    投稿詳細画面[0m
  アイコンのテスト
    トップ画面
[32m      本のアイコンが表示される[0m
    アバウト画面
[32m      本のアイコンが表示される[0m
    ヘッダー: ログインしていない場合
[32m      Homeリンクのアイコンが表示される[0m
[32m      Aboutリンクのアイコンが表示される[0m
[32m      sign upリンクのアイコンが表示される[0m
[32m      loginリンクのアイコンが表示される[0m
    ヘッダー: ログインしている場合
[32m      Homeリンクのアイコンが表示される[0m
[32m      Usersリンクのアイコンが表示される[0m
[32m      Booksリンクのアイコンが表示される[0m
[32m      log outリンクのアイコンが表示される[0m
    サイドバー
[32m      ユーザ一覧画面でレンチアイコンが表示される[0m
[32m      ユーザ詳細画面でレンチアイコンが表示される[0m
[32m      投稿一覧画面でレンチアイコンが表示される[0m
[32m      投稿詳細画面でレンチアイコンが表示される[0m

投稿のテスト
  トップ画面(root_path)のテスト
    表示の確認
[31m      トップ画面(root_path)に一覧ページへのリンクが表示されているか (FAILED - 8)[0m
[32m      root_pathが"/"であるか[0m
  一覧画面のテスト
    一覧の表示とリンクの確認
[31m      bookの一覧表示(tableタグ)と投稿フォームが同一画面に表示されているか (FAILED - 9)[0m
[31m      bookのタイトルと感想を表示し、詳細・編集・削除のリンクが表示されているか (FAILED - 10)[0m
[31m      Create Bookボタンが表示される (FAILED - 11)[0m
    投稿処理に関するテスト
[31m      投稿に成功しサクセスメッセージが表示されるか (FAILED - 12)[0m
[31m      投稿に失敗する (FAILED - 13)[0m
[31m      投稿後のリダイレクト先は正しいか (FAILED - 14)[0m
    book削除のテスト
[32m      bookの削除[0m
  詳細画面のテスト
    表示の確認
[31m      本のタイトルと感想が画面に表示されていること (FAILED - 15)[0m
[31m      Editリンクが表示される (FAILED - 16)[0m
[31m      Backリンクが表示される (FAILED - 17)[0m
    リンクの遷移先の確認
[31m      Editの遷移先は編集画面か (FAILED - 18)[0m
[31m      Backの遷移先は一覧画面か (FAILED - 19)[0m
  編集画面のテスト
    表示の確認
[31m      編集前のタイトルと感想がフォームに表示(セット)されている (FAILED - 20)[0m
[31m      Update Bookボタンが表示される (FAILED - 21)[0m
[31m      Showリンクが表示される (FAILED - 22)[0m
[31m      Backリンクが表示される (FAILED - 23)[0m
    リンクの遷移先の確認
[31m      Showの遷移先は詳細画面か (FAILED - 24)[0m
[31m      Backの遷移先は一覧画面か (FAILED - 25)[0m
    更新処理に関するテスト
[31m      更新に成功しサクセスメッセージが表示されるか (FAILED - 26)[0m
[31m      更新に失敗しエラーメッセージが表示されるか (FAILED - 27)[0m
[31m      更新後のリダイレクト先は正しいか (FAILED - 28)[0m

Failures:

  1) [STEP2] ユーザログイン後のテスト ユーザ一覧画面のテスト 表示内容の確認 自分と他人の画像が表示される: fallbackの画像がサイドバーの1つ＋一覧(2人)の2つの計3つ存在する
     [31mFailure/Error: expect(all('img').size).to eq(3)[0m
     [31m[0m
     [31m  expected: 3[0m
     [31m       got: 4[0m
     [31m[0m
     [31m  (compared using ==)[0m
     [36m# ./spec/system/02_after_login_spec.rb:258:in `block (4 levels) in <main>'[0m

  2) [STEP2] ユーザログイン後のテスト 自分のユーザ情報編集画面のテスト 表示の確認 名前編集フォームに自分の名前が表示される
     [31mFailure/Error: expect(page).to have_field 'user[name]', with: user.name[0m
     [31m  expected to find visible field "user[name]" that is not disabled with value "izyoh2jptm" but there were no matches. Also found "", which matched the selector but not all filters. Expected value to be "izyoh2jptm" but was nil[0m
     [36m# ./spec/system/02_after_login_spec.rb:363:in `block (4 levels) in <main>'[0m

  3) [STEP2] ユーザログイン後のテスト 自分のユーザ情報編集画面のテスト 表示の確認 自己紹介編集フォームに自分の自己紹介文が表示される
     [31mFailure/Error: expect(page).to have_field 'user[introduction]', with: user.introduction[0m
     [31m  expected to find visible field "user[introduction]" that is not disabled with value "3943wk7scaeo9i8qtoh8" but there were no matches. Also found "", which matched the selector but not all filters. Expected value to be "3943wk7scaeo9i8qtoh8" but was ""[0m
     [36m# ./spec/system/02_after_login_spec.rb:369:in `block (4 levels) in <main>'[0m

  4) [STEP3] 仕上げのテスト サクセスメッセージのテスト ユーザのプロフィール情報更新成功時
     [31mFailure/Error: is_expected.to have_content 'successfully'[0m
     [31m  expected to find text "successfully" in "Bookers\nHome Users Books Logout 　 　　　　 　　　\n　 　 　\n1 errors prohibited this object from being saved\nName is too short (minimum is 2 characters)\nUser Info\nName\nImage\nIntroduction\nShow | Back\nCopyRight KenInoue.inc"[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:45:in `block (3 levels) in <main>'[0m

  5) [STEP3] 仕上げのテスト 処理失敗時のテスト ユーザのプロフィール情報編集失敗: nameを1文字にする ユーザ編集画面を表示しており、フォームの内容が正しい
     [31mFailure/Error: expect(page).to have_field 'user[name]', with: @name[0m
     [31m  expected to find visible field "user[name]" that is not disabled with value "q" but there were no matches. Also found "", which matched the selector but not all filters. Expected value to be "q" but was nil[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:113:in `block (4 levels) in <main>'[0m

  6) [STEP3] 仕上げのテスト 他人の画面のテスト 他人の投稿詳細画面のテスト サイドバーの確認 他人のユーザ編集画面へのリンクが存在する
     [31mFailure/Error: expect(page).to have_link '', href: edit_user_path(other_user)[0m
     [31m  expected to find link "" with href "/users/1/edit" but there were no matches[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:248:in `block (5 levels) in <main>'[0m

  7) [STEP3] 仕上げのテスト 他人の画面のテスト 他人のユーザ詳細画面のテスト サイドバーの確認 他人のユーザ編集画面へのリンクが存在する
     [31mFailure/Error: expect(page).to have_link '', href: edit_user_path(other_user)[0m
     [31m  expected to find link "" with href "/users/1/edit" but there were no matches[0m
     [36m# ./spec/system/03_finishing_touches_spec.rb:297:in `block (5 levels) in <main>'[0m

  8) 投稿のテスト トップ画面(root_path)のテスト 表示の確認 トップ画面(root_path)に一覧ページへのリンクが表示されているか
     [31mFailure/Error: expect(page).to have_link "", href: books_path[0m
     [31m  expected to find link "" with href "/books" but there were no matches[0m
     [36m# ./spec/system/books_spec.rb:11:in `block (4 levels) in <main>'[0m

  9) 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 bookの一覧表示(tableタグ)と投稿フォームが同一画面に表示されているか
     [31mFailure/Error: expect(page).to have_selector 'table'[0m
     [31m  expected to find css "table" but there were no matches[0m
     [36m# ./spec/system/books_spec.rb:24:in `block (4 levels) in <main>'[0m

  10) 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 bookのタイトルと感想を表示し、詳細・編集・削除のリンクが表示されているか
      [31mFailure/Error: expect(page).to have_content book.title[0m
      [31m  expected to find text "hoge" in "Bookers\nHome About Sign UP LogIn 　　　　　　　 　　　　 　　　\n　 　 　\nLog in\nName\nPassword\nRemember me\nSign up Forgot your password?\nCopyRight KenInoue.inc"[0m
      [36m# ./spec/system/books_spec.rb:35:in `block (5 levels) in <main>'[0m
      [36m# ./spec/system/books_spec.rb:33:in `each_with_index'[0m
      [36m# ./spec/system/books_spec.rb:33:in `block (4 levels) in <main>'[0m

  11) 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 Create Bookボタンが表示される
      [31mFailure/Error: expect(page).to have_button 'Create Book'[0m
      [31m  expected to find button "Create Book" that is not disabled but there were no matches[0m
      [36m# ./spec/system/books_spec.rb:52:in `block (4 levels) in <main>'[0m

  12) 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿に成功しサクセスメッセージが表示されるか
      [31mFailure/Error: fill_in 'book[title]', with: Faker::Lorem.characters(number:5)[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find field "book[title]" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:91:in `fill_in'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:57:in `block (4 levels) in <main>'[0m

  13) 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿に失敗する
      [31mFailure/Error: click_button 'Create Book'[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find button "Create Book" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:58:in `click_button'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:63:in `block (4 levels) in <main>'[0m

  14) 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿後のリダイレクト先は正しいか
      [31mFailure/Error: fill_in 'book[title]', with: Faker::Lorem.characters(number:5)[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find field "book[title]" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:91:in `fill_in'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:68:in `block (4 levels) in <main>'[0m

  15) 投稿のテスト 詳細画面のテスト 表示の確認 本のタイトルと感想が画面に表示されていること
      [31mFailure/Error: expect(page).to have_content book.title[0m
      [31m  expected to find text "hoge" in "Bookers\nHome About Sign UP LogIn 　　　　　　　 　　　　 　　　\n　 　 　\nLog in\nName\nPassword\nRemember me\nSign up Forgot your password?\nCopyRight KenInoue.inc"[0m
      [36m# ./spec/system/books_spec.rb:87:in `block (4 levels) in <main>'[0m

  16) 投稿のテスト 詳細画面のテスト 表示の確認 Editリンクが表示される
      [31mFailure/Error: expect(edit_link.native.inner_text).to match(/edit/i)[0m
      [31m[0m
      [31m  expected " Bookers" to match /edit/i[0m
      [31m  Diff:[0m[0m
      [31m  [0m[34m@@ -1 +1 @@[0m
      [31m  [0m[31m-/edit/i[0m
      [31m  [0m[32m+" Bookers"[0m
      [31m  [0m[0m
      [36m# ./spec/system/books_spec.rb:92:in `block (4 levels) in <main>'[0m

  17) 投稿のテスト 詳細画面のテスト 表示の確認 Backリンクが表示される
      [31mFailure/Error: expect(back_link.native.inner_text).to match(/back/i)[0m
      [31m[0m
      [31m  expected "Home" to match /back/i[0m
      [31m  Diff:[0m[0m
      [31m  [0m[34m@@ -1 +1 @@[0m
      [31m  [0m[31m-/back/i[0m
      [31m  [0m[32m+"Home"[0m
      [31m  [0m[0m
      [36m# ./spec/system/books_spec.rb:96:in `block (4 levels) in <main>'[0m

  18) 投稿のテスト 詳細画面のテスト リンクの遷移先の確認 Editの遷移先は編集画面か
      [31mFailure/Error: expect(current_path).to eq('/books/' + book.id.to_s + '/edit')[0m
      [31m[0m
      [31m  expected: "/books/1/edit"[0m
      [31m       got: "/"[0m
      [31m[0m
      [31m  (compared using ==)[0m
      [36m# ./spec/system/books_spec.rb:103:in `block (4 levels) in <main>'[0m

  19) 投稿のテスト 詳細画面のテスト リンクの遷移先の確認 Backの遷移先は一覧画面か
      [31mFailure/Error: expect(page).to have_current_path books_path[0m
      [31m  expected "/" to equal "/books"[0m
      [36m# ./spec/system/books_spec.rb:108:in `block (4 levels) in <main>'[0m

  20) 投稿のテスト 編集画面のテスト 表示の確認 編集前のタイトルと感想がフォームに表示(セット)されている
      [31mFailure/Error: expect(page).to have_field 'book[title]', with: book.title[0m
      [31m  expected to find field "book[title]" that is not disabled but there were no matches[0m
      [36m# ./spec/system/books_spec.rb:118:in `block (4 levels) in <main>'[0m

  21) 投稿のテスト 編集画面のテスト 表示の確認 Update Bookボタンが表示される
      [31mFailure/Error: expect(page).to have_button 'Update Book'[0m
      [31m  expected to find button "Update Book" that is not disabled but there were no matches[0m
      [36m# ./spec/system/books_spec.rb:122:in `block (4 levels) in <main>'[0m

  22) 投稿のテスト 編集画面のテスト 表示の確認 Showリンクが表示される
      [31mFailure/Error: expect(show_link.native.inner_text).to match(/show/i)[0m
      [31m[0m
      [31m  expected " Bookers" to match /show/i[0m
      [31m  Diff:[0m[0m
      [31m  [0m[34m@@ -1 +1 @@[0m
      [31m  [0m[31m-/show/i[0m
      [31m  [0m[32m+" Bookers"[0m
      [31m  [0m[0m
      [36m# ./spec/system/books_spec.rb:126:in `block (4 levels) in <main>'[0m

  23) 投稿のテスト 編集画面のテスト 表示の確認 Backリンクが表示される
      [31mFailure/Error: expect(back_link.native.inner_text).to match(/back/i)[0m
      [31m[0m
      [31m  expected "Home" to match /back/i[0m
      [31m  Diff:[0m[0m
      [31m  [0m[34m@@ -1 +1 @@[0m
      [31m  [0m[31m-/back/i[0m
      [31m  [0m[32m+"Home"[0m
      [31m  [0m[0m
      [36m# ./spec/system/books_spec.rb:130:in `block (4 levels) in <main>'[0m

  24) 投稿のテスト 編集画面のテスト リンクの遷移先の確認 Showの遷移先は詳細画面か
      [31mFailure/Error: expect(current_path).to eq('/books/' + book.id.to_s)[0m
      [31m[0m
      [31m  expected: "/books/1"[0m
      [31m       got: "/"[0m
      [31m[0m
      [31m  (compared using ==)[0m
      [36m# ./spec/system/books_spec.rb:137:in `block (4 levels) in <main>'[0m

  25) 投稿のテスト 編集画面のテスト リンクの遷移先の確認 Backの遷移先は一覧画面か
      [31mFailure/Error: expect(page).to have_current_path books_path[0m
      [31m  expected "/" to equal "/books"[0m
      [36m# ./spec/system/books_spec.rb:142:in `block (4 levels) in <main>'[0m

  26) 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新に成功しサクセスメッセージが表示されるか
      [31mFailure/Error: fill_in 'book[title]', with: Faker::Lorem.characters(number:5)[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find field "book[title]" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:91:in `fill_in'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:147:in `block (4 levels) in <main>'[0m

  27) 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新に失敗しエラーメッセージが表示されるか
      [31mFailure/Error: fill_in 'book[title]', with: ""[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find field "book[title]" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:91:in `fill_in'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:153:in `block (4 levels) in <main>'[0m

  28) 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新後のリダイレクト先は正しいか
      [31mFailure/Error: fill_in 'book[title]', with: Faker::Lorem.characters(number:5)[0m
      [31m[0m
      [31mCapybara::ElementNotFound:[0m
      [31m  Unable to find field "book[title]" that is not disabled[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:303:in `block in synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/base.rb:83:in `synchronize'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:292:in `synced_resolve'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/finders.rb:53:in `find'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/node/actions.rb:91:in `fill_in'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/session.rb:768:in `block (2 levels) in <class:Session>'[0m
      [36m# /home/ec2-user/.rvm/gems/ruby-2.6.3/gems/capybara-3.35.3/lib/capybara/dsl.rb:58:in `block (2 levels) in <module:DSL>'[0m
      [36m# ./spec/system/books_spec.rb:159:in `block (4 levels) in <main>'[0m

Finished in 17.11 seconds (files took 1.48 seconds to load)
[31m220 examples, 28 failures[0m

Failed examples:

[31mrspec ./spec/system/02_after_login_spec.rb:257[0m [36m# [STEP2] ユーザログイン後のテスト ユーザ一覧画面のテスト 表示内容の確認 自分と他人の画像が表示される: fallbackの画像がサイドバーの1つ＋一覧(2人)の2つの計3つ存在する[0m
[31mrspec ./spec/system/02_after_login_spec.rb:362[0m [36m# [STEP2] ユーザログイン後のテスト 自分のユーザ情報編集画面のテスト 表示の確認 名前編集フォームに自分の名前が表示される[0m
[31mrspec ./spec/system/02_after_login_spec.rb:368[0m [36m# [STEP2] ユーザログイン後のテスト 自分のユーザ情報編集画面のテスト 表示の確認 自己紹介編集フォームに自分の自己紹介文が表示される[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:38[0m [36m# [STEP3] 仕上げのテスト サクセスメッセージのテスト ユーザのプロフィール情報更新成功時[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:112[0m [36m# [STEP3] 仕上げのテスト 処理失敗時のテスト ユーザのプロフィール情報編集失敗: nameを1文字にする ユーザ編集画面を表示しており、フォームの内容が正しい[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:247[0m [36m# [STEP3] 仕上げのテスト 他人の画面のテスト 他人の投稿詳細画面のテスト サイドバーの確認 他人のユーザ編集画面へのリンクが存在する[0m
[31mrspec ./spec/system/03_finishing_touches_spec.rb:296[0m [36m# [STEP3] 仕上げのテスト 他人の画面のテスト 他人のユーザ詳細画面のテスト サイドバーの確認 他人のユーザ編集画面へのリンクが存在する[0m
[31mrspec ./spec/system/books_spec.rb:10[0m [36m# 投稿のテスト トップ画面(root_path)のテスト 表示の確認 トップ画面(root_path)に一覧ページへのリンクが表示されているか[0m
[31mrspec ./spec/system/books_spec.rb:23[0m [36m# 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 bookの一覧表示(tableタグ)と投稿フォームが同一画面に表示されているか[0m
[31mrspec ./spec/system/books_spec.rb:28[0m [36m# 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 bookのタイトルと感想を表示し、詳細・編集・削除のリンクが表示されているか[0m
[31mrspec ./spec/system/books_spec.rb:51[0m [36m# 投稿のテスト 一覧画面のテスト 一覧の表示とリンクの確認 Create Bookボタンが表示される[0m
[31mrspec ./spec/system/books_spec.rb:56[0m [36m# 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿に成功しサクセスメッセージが表示されるか[0m
[31mrspec ./spec/system/books_spec.rb:62[0m [36m# 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿に失敗する[0m
[31mrspec ./spec/system/books_spec.rb:67[0m [36m# 投稿のテスト 一覧画面のテスト 投稿処理に関するテスト 投稿後のリダイレクト先は正しいか[0m
[31mrspec ./spec/system/books_spec.rb:86[0m [36m# 投稿のテスト 詳細画面のテスト 表示の確認 本のタイトルと感想が画面に表示されていること[0m
[31mrspec ./spec/system/books_spec.rb:90[0m [36m# 投稿のテスト 詳細画面のテスト 表示の確認 Editリンクが表示される[0m
[31mrspec ./spec/system/books_spec.rb:94[0m [36m# 投稿のテスト 詳細画面のテスト 表示の確認 Backリンクが表示される[0m
[31mrspec ./spec/system/books_spec.rb:100[0m [36m# 投稿のテスト 詳細画面のテスト リンクの遷移先の確認 Editの遷移先は編集画面か[0m
[31mrspec ./spec/system/books_spec.rb:105[0m [36m# 投稿のテスト 詳細画面のテスト リンクの遷移先の確認 Backの遷移先は一覧画面か[0m
[31mrspec ./spec/system/books_spec.rb:117[0m [36m# 投稿のテスト 編集画面のテスト 表示の確認 編集前のタイトルと感想がフォームに表示(セット)されている[0m
[31mrspec ./spec/system/books_spec.rb:121[0m [36m# 投稿のテスト 編集画面のテスト 表示の確認 Update Bookボタンが表示される[0m
[31mrspec ./spec/system/books_spec.rb:124[0m [36m# 投稿のテスト 編集画面のテスト 表示の確認 Showリンクが表示される[0m
[31mrspec ./spec/system/books_spec.rb:128[0m [36m# 投稿のテスト 編集画面のテスト 表示の確認 Backリンクが表示される[0m
[31mrspec ./spec/system/books_spec.rb:134[0m [36m# 投稿のテスト 編集画面のテスト リンクの遷移先の確認 Showの遷移先は詳細画面か[0m
[31mrspec ./spec/system/books_spec.rb:139[0m [36m# 投稿のテスト 編集画面のテスト リンクの遷移先の確認 Backの遷移先は一覧画面か[0m
[31mrspec ./spec/system/books_spec.rb:146[0m [36m# 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新に成功しサクセスメッセージが表示されるか[0m
[31mrspec ./spec/system/books_spec.rb:152[0m [36m# 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新に失敗しエラーメッセージが表示されるか[0m
[31mrspec ./spec/system/books_spec.rb:158[0m [36m# 投稿のテスト 編集画面のテスト 更新処理に関するテスト 更新後のリダイレクト先は正しいか[0m

