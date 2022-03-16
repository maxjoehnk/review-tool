import 'package:cached_network_image/cached_network_image.dart';
import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';

const double userAvatarSize = 32;

class UserAvatar extends StatelessWidget {
  final User user;

  const UserAvatar(this.user, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        width: userAvatarSize,
        height: userAvatarSize,
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(userAvatarSize),
          color: Colors.green,
        ),
        clipBehavior: Clip.antiAlias,
        child: user.avatarUrl == null
            ? Center(child: Text(user.name.isNotEmpty ? user.name[0] : ""))
            : CachedNetworkImage(
                imageUrl: user.avatarUrl!,
                placeholder: (context, url) => Center(child: Text(user.name[0])),
                errorWidget: (context, url, error) => Center(child: Text(user.name[0])),
              ));
  }
}
