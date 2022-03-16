import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart' hide ReviewState;
import 'package:review_tool/state/review_state.dart';

import 'discussions.dart';
import 'file_tree.dart';
import 'file_view.dart';
import 'overview.dart';
import 'review_card.dart';

class ReviewScreen extends StatelessWidget {
  final Review review;
  final Function() onClose;

  const ReviewScreen(this.review, {required this.onClose, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocProvider(
      create: (context) => ReviewCubit(context.read(), review),
      child: ReviewView(onClose: onClose),
    );
  }
}

class ReviewView extends StatelessWidget {
  final Function() onClose;

  const ReviewView({required this.onClose, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    context.read<ReviewCubit>().fetch();
    var textTheme = Theme.of(context).textTheme;
    return BlocBuilder<ReviewCubit, ReviewState>(
      builder: (context, state) => Column(
          mainAxisSize: MainAxisSize.min,
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            ReviewCard(onClose: onClose),
            Expanded(
              child: Row(
                  crossAxisAlignment: CrossAxisAlignment.start,
                  mainAxisSize: MainAxisSize.max,
                  children: [
                    Flexible(
                        flex: 1,
                        fit: FlexFit.tight,
                        child: Padding(
                          padding: const EdgeInsets.symmetric(horizontal: 8),
                          child: Column(
                              crossAxisAlignment: CrossAxisAlignment.start,
                              mainAxisSize: MainAxisSize.min,
                              children: [
                                const ReviewOverview(),
                                const Padding(padding: EdgeInsets.all(8)),
                                Row(
                                  mainAxisSize: MainAxisSize.max,
                                  children: [
                                    Padding(
                                      padding: const EdgeInsets.only(right: 8.0),
                                      child: Text("Review Summary", style: textTheme.titleSmall),
                                    ),
                                    const Expanded(child: Divider()),
                                  ],
                                ),
                                const Expanded(child: ReviewFileTree()),
                              ]),
                        )),
                    const Padding(padding: EdgeInsets.all(8)),
                    Expanded(
                        flex: 3,
                        child: state.selectedFile == null
                            ? const ReviewDiscussions()
                            : ReviewFileView(state.review, state.selectedFile!)),
                  ]),
            )
          ]),
    );
  }
}
