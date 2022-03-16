import 'package:flutter/material.dart';

import 'api.dart';
import 'ffi.dart';
import 'service_binder.dart';
import 'settings.dart';
import 'ui/nav_bar.dart';
import 'ui/views/review/review.dart';
import 'ui/views/review_list/review_list_screen.dart';
import 'ui/views/settings/settings.dart';

void main() async {
  Settings settings = await Settings.load();
  await api.configureModules(modules: settings.providers);

  runApp(ReviewTool(settings));
}

class ReviewTool extends StatelessWidget {
  final Settings settings;

  const ReviewTool(this.settings, {Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Review Tool',
      theme: ThemeData(
        visualDensity: VisualDensity.compact,
        primarySwatch: Colors.blue,
        brightness: Brightness.dark,
      ),
      home: ServiceBinder(settings: settings, child: const Scaffold(body: Router())),
    );
  }
}

class Router extends StatefulWidget {
  const Router({Key? key}) : super(key: key);

  @override
  State<Router> createState() => _RouterState();
}

class _RouterState extends State<Router> {
  Review? review;

  @override
  Widget build(BuildContext context) {
    return NavBar(
      settings: const SettingsScreen(),
      view: (context) =>
          ProviderRoutes(review, onSelectReview: (review) => setState(() => this.review = review)),
    );
  }
}

class ProviderRoutes extends StatelessWidget {
  final Review? review;
  final Function(Review?) onSelectReview;

  const ProviderRoutes(this.review, {required this.onSelectReview, Key? key}) : super(key: key);

  @override
  Widget build(BuildContext context) {
    if (review != null) {
      return ReviewScreen(review!, onClose: () => onSelectReview(null));
    }
    return ReviewListScreen(onSelectReview: onSelectReview);
  }
}
