import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/state/review_state.dart';
import 'package:review_tool/ui/widgets/user_list_item.dart';

class ReviewOverview extends StatelessWidget {
  const ReviewOverview({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    var textTheme = Theme.of(context).textTheme;
    return BlocBuilder<ReviewCubit, ReviewState>(
      builder: (context, state) => Column(
        mainAxisSize: MainAxisSize.min,
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 8),
            child: Text("Authors", style: textTheme.titleSmall),
          ),
          Wrap(
              spacing: 8,
              runSpacing: 4,
              children: state.review.authors.map((u) => UserListItem(u)).toList()),
          Padding(
            padding: const EdgeInsets.symmetric(vertical: 8),
            child: Text("Reviewers", style: textTheme.titleSmall),
          ),
          Wrap(
              spacing: 8,
              runSpacing: 4,
              children: state.review.reviewers.map((u) => UserListItem(u)).toList()),
        ],
      ),
    );
  }
}
