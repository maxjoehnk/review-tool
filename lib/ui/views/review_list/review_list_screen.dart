import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart';
import 'package:review_tool/state/review_list_state.dart';

import 'review_list.dart';

class ReviewListScreen extends StatelessWidget {
  final Function(Review?) onSelectReview;

  const ReviewListScreen({required this.onSelectReview, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    context.read<ReviewListState>().fetchReviews();
    return BlocBuilder<ReviewListState, RemoteList<Review>>(
      builder: (context, reviews) {
        if (reviews.pending) {
          return const ReviewLoadingList();
        }
        return ReviewList(reviews: reviews, onSelectReview: (review) => onSelectReview(review));
      },
    );
  }
}

class ReviewLoadingList extends StatelessWidget {
  const ReviewLoadingList({Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container();
  }
}
