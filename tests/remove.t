Test removing owners with modowners.

Setup test data:

  $ mkdir .github
  $ echo "# Testing modowners-remove"  >> .github/CODEOWNERS
  $ echo "* @foo @bar @baz @bar"       >> .github/CODEOWNERS
  $ echo                               >> .github/CODEOWNERS
  $ echo "# Some more rules..."        >> .github/CODEOWNERS
  $ echo                               >> .github/CODEOWNERS
  $ echo "#  ___________________"      >> .github/CODEOWNERS
  $ echo "# < This is the ways! >"     >> .github/CODEOWNERS
  $ echo "#  -------------------"      >> .github/CODEOWNERS
  $ echo "#         \\"                >> .github/CODEOWNERS
  $ echo "#          \\"               >> .github/CODEOWNERS
  $ echo "#             _~^~^~_"       >> .github/CODEOWNERS
  $ echo "#         \) /  o o  \\ (/"  >> .github/CODEOWNERS
  $ echo "#           '_   -   _'"     >> .github/CODEOWNERS
  $ echo "#           / '-----' \\"    >> .github/CODEOWNERS
  $ echo "*.rs @ferris # 🦀"           >> .github/CODEOWNERS
  $ echo "# Owns text files"           >> .github/CODEOWNERS
  $ echo "*.txt @myorg/myteam"         >> .github/CODEOWNERS
  $ cat .github/CODEOWNERS
  # Testing modowners-remove
  * @foo @bar @baz @bar
  
  # Some more rules...
  
  #  ___________________
  # < This is the ways! >
  #  -------------------
  #         \
  #          \
  #             _~^~^~_
  #         \) /  o o  \ (/
  #           '_   -   _'
  #           / '-----' \
  *.rs @ferris # 🦀
  # Owns text files
  *.txt @myorg/myteam

It should remove a single owner from an entry, even if it is duplicated.

  $ modowners remove '*' @bar
  $ cat .github/CODEOWNERS
  # Testing modowners-remove
  * @foo @baz
  
  # Some more rules...
  
  #  ___________________
  # < This is the ways! >
  #  -------------------
  #         \
  #          \
  #             _~^~^~_
  #         \) /  o o  \ (/
  #           '_   -   _'
  #           / '-----' \
  *.rs @ferris # 🦀
  # Owns text files
  *.txt @myorg/myteam

If there's only one owner, it should remove the entry completely, including
directly preceeding and inline comments.

  $ modowners remove '*.rs' @ferris
  $ cat .github/CODEOWNERS
  # Testing modowners-remove
  * @foo @baz
  
  # Some more rules...
  
  # Owns text files
  *.txt @myorg/myteam
