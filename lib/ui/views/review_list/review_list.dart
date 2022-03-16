import 'package:flutter/material.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/state/review_list_state.dart';
import 'package:review_tool/ui/widgets/review_open_label.dart';
import 'package:review_tool/ui/widgets/user_avatar.dart';

class ReviewList extends StatelessWidget {
  final RemoteList<Review> reviews;
  final Function(Review) onSelectReview;

  const ReviewList({required this.reviews, required this.onSelectReview, Key? key})
      : super(key: key);

  @override
  Widget build(BuildContext context) {
    return ListView.builder(
      itemCount: reviews.list.length,
      itemBuilder: (context, i) {
        var review = reviews.list[i];

        return ReviewListItem(review, onSelect: () => onSelectReview(review));
      },
    );
  }
}

class ReviewListItem extends StatelessWidget {
  final Review review;
  final Function() onSelect;

  const ReviewListItem(this.review, {required this.onSelect, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var theme = Theme.of(context).textTheme;
    return GestureDetector(
      behavior: HitTestBehavior.translucent,
      onTap: onSelect,
      child: Padding(
        padding: const EdgeInsets.all(8.0),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            Row(children: [
              Expanded(
                  child: Row(
                mainAxisSize: MainAxisSize.min,
                children: [
                  ReviewOpenLabel(open: review.open),
                  const Padding(padding: EdgeInsets.all(4)),
                  Text(review.title, style: theme.titleMedium),
                ],
              )),
              Text(review.branchName,
                  style: const TextStyle(color: Colors.white54), textAlign: TextAlign.end),
            ]),
            Text("Authors: ${review.authors.map((u) => u.name).join(", ")}",
                style: theme.bodySmall),
            const Padding(padding: EdgeInsets.all(4)),
            Row(
                children: review.reviewers
                    .map((u) =>
                        Padding(padding: const EdgeInsets.only(right: 8), child: UserAvatar(u)))
                    .toList()),
          ],
        ),
      ),
    );
  }
}
