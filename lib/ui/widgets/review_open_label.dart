import 'package:flutter/material.dart';

class ReviewOpenLabel extends StatelessWidget {
  final bool open;

  const ReviewOpenLabel({required this.open, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return Container(
        decoration: BoxDecoration(
          borderRadius: BorderRadius.circular(2),
          color: open ? Colors.blue : Colors.grey,
        ),
        padding: const EdgeInsets.symmetric(vertical: 2, horizontal: 4),
        child: Text(open ? "Open" : "Closed"));
  }
}
