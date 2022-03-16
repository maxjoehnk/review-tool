import 'package:flutter/material.dart';
import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/state/review_state.dart';
import 'package:review_tool/ui/widgets/review_open_label.dart';

class ReviewCard extends StatelessWidget {
  final Function() onClose;

  const ReviewCard({required this.onClose, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return BlocBuilder<ReviewCubit, ReviewState>(
      builder: (context, state) => Card(
        child: Column(mainAxisSize: MainAxisSize.min, children: [
          Padding(
            padding: const EdgeInsets.all(0),
            child: Row(children: [
              IconButton(
                  onPressed: onClose,
                  icon: const Icon(Icons.arrow_back),
                  splashRadius: 16,
                  padding: const EdgeInsets.all(0)),
              ReviewOpenLabel(open: state.review.open),
              const Padding(padding: EdgeInsets.all(4)),
              Text(state.review.title)
            ]),
          ),
          const Divider(color: Colors.white54, height: 2),
          Padding(
            padding: const EdgeInsets.all(8.0),
            child: Row(children: [
              ElevatedButton(
                  child: const Text("Accept Review"),
                  onPressed: () {},
                  style: ButtonStyle(backgroundColor: AcceptReviewButtonColor())),
              const Padding(padding: EdgeInsets.all(4)),
              OutlinedButton(
                  child: const Text("Reject Review"),
                  onPressed: () {},
                  style: ButtonStyle(foregroundColor: RejectReviewButtonColor())),
            ]),
          )
        ]),
      ),
    );
  }
}

class AcceptReviewButtonColor extends MaterialStateColor {
  AcceptReviewButtonColor() : super(Colors.green.value);

  @override
  Color resolve(Set<MaterialState> states) {
    return Colors.green;
  }
}

class RejectReviewButtonColor extends MaterialStateColor {
  RejectReviewButtonColor() : super(Colors.red.value);

  @override
  Color resolve(Set<MaterialState> states) {
    return Colors.red;
  }
}
