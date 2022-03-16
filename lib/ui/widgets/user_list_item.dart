import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/ui/widgets/user_avatar.dart';

class UserListItem extends StatelessWidget {
  final User user;

  const UserListItem(this.user, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Row(
      mainAxisSize: MainAxisSize.min,
      children: [UserAvatar(user), const Padding(padding: EdgeInsets.all(4)), Text(user.name)],
    );
  }
}
