import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:flutter_markdown/flutter_markdown.dart';
import 'package:markdown/markdown.dart' as md;
import 'package:review_tool/api.dart' hide ReviewState;
import 'package:review_tool/state/review_state.dart';
import 'package:review_tool/ui/widgets/file_type_icon.dart';
import 'package:review_tool/ui/widgets/user_avatar.dart';
import 'package:shimmer/shimmer.dart';
import 'package:timeago/timeago.dart' as timeago;

class ReviewDiscussions extends StatelessWidget {
  const ReviewDiscussions({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<ReviewCubit, ReviewState>(builder: (context, state) {
      if (state.loading) {
        return ListView.builder(
            itemBuilder: (context, i) => const DiscussionLoadingView(), itemCount: 5);
      }
      return ListView(
        children: state.discussions.map((discussion) => ReviewDiscussionView(discussion)).toList(),
      );
    });
  }
}

class ReviewDiscussionView extends StatelessWidget {
  final ReviewDiscussion discussion;

  const ReviewDiscussionView(this.discussion, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textTheme = Theme.of(context).textTheme;
    return DiscussionBaseView(
        fileRow: Row(children: [
          FileType(filename: discussion.file!.fileName),
          const Padding(padding: EdgeInsets.all(2)),
          GestureDetector(
              child: Text(discussion.file!.fileName),
              onTap: () => context.read<ReviewCubit>().selectFile(
                    discussion.file!.filePath,
                    discussion.file!.revision!,
                  )),
          const Padding(padding: EdgeInsets.all(2)),
          if (discussion.file!.revision != null)
            Text("revision ${discussion.file!.revision!.substring(0, 7)}",
                style: textTheme.bodySmall)
        ]),
        comments: discussion.comments
            .map((comment) => Padding(
                  padding: const EdgeInsets.only(left: 24.0),
                  child: DiscussionComment(comment),
                ))
            .toList());
  }
}

class DiscussionComment extends StatelessWidget {
  final ReviewComment comment;

  const DiscussionComment(this.comment, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textTheme = Theme.of(context).textTheme;
    return DiscussionBaseComment(
        avatar: UserAvatar(comment.user),
        header: Row(mainAxisSize: MainAxisSize.min, children: [
          Text(comment.user.name, style: textTheme.titleSmall),
          const Padding(padding: EdgeInsets.all(4)),
          Text(timeago.format(DateTime.fromMillisecondsSinceEpoch(comment.timestamp)),
              style: textTheme.bodySmall)
        ]),
        body: MarkdownBody(
            data: comment.text,
            extensionSet: md.ExtensionSet(
              md.ExtensionSet.gitHubFlavored.blockSyntaxes,
              [md.EmojiSyntax(), ...md.ExtensionSet.gitHubFlavored.inlineSyntaxes],
            )));
  }
}

class DiscussionLoadingView extends StatelessWidget {
  const DiscussionLoadingView({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Shimmer.fromColors(
      child: DiscussionBaseView(
        fileRow: const TextShimmer(height: 14, width: 256),
        comments: List.generate(2, (index) => const DiscussionLoadingComment()),
      ),
      baseColor: Colors.white24,
      highlightColor: Colors.white54,
    );
  }
}

class DiscussionLoadingComment extends StatelessWidget {
  const DiscussionLoadingComment({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return DiscussionBaseComment(
        avatar: Container(
          width: userAvatarSize,
          height: userAvatarSize,
          decoration: BoxDecoration(
            borderRadius: BorderRadius.circular(userAvatarSize),
            color: Colors.white,
          ),
          clipBehavior: Clip.antiAlias,
        ),
        header: const TextShimmer(width: 256, height: 12),
        body: Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: const [
            TextShimmer(width: 512, height: 12),
          ],
        ));
  }
}

class TextShimmer extends StatelessWidget {
  final double height;
  final double width;

  const TextShimmer({this.height = 16, this.width = 128, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
      margin: const EdgeInsets.symmetric(vertical: 3),
      height: height,
      width: width,
      decoration: BoxDecoration(
        borderRadius: BorderRadius.circular(2),
        color: Colors.white,
      ),
    );
  }
}

class DiscussionBaseView extends StatelessWidget {
  final Widget fileRow;
  final List<Widget> comments;

  const DiscussionBaseView({required this.fileRow, required this.comments, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        mainAxisSize: MainAxisSize.min,
        children: [
          fileRow,
          comments[0],
          ...comments.skip(1).map((comment) => Padding(
                padding: const EdgeInsets.only(left: 24.0),
                child: comment,
              )),
          const Divider(),
        ]);
  }
}

class DiscussionBaseComment extends StatelessWidget {
  final Widget avatar;
  final Widget header;
  final Widget body;

  const DiscussionBaseComment(
      {required this.avatar, required this.header, required this.body, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Padding(
      padding: const EdgeInsets.symmetric(vertical: 4.0),
      child: Row(children: [
        avatar,
        const Padding(padding: EdgeInsets.all(8)),
        Expanded(
          child: Column(
              crossAxisAlignment: CrossAxisAlignment.start,
              mainAxisSize: MainAxisSize.min,
              children: [header, body]),
        )
      ]),
    );
  }
}
