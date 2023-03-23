import 'bridge_generated.dart';

// Re-export the bridge so it is only necessary to import this file.
export 'bridge_generated.dart';

class ProviderApi {
  final Native api;
  final String providerId;

  ProviderApi(this.api, this.providerId);

  Future<List<Review>> getReviews({dynamic hint}) {
    return api.getReviews(providerId: providerId, hint: hint);
  }

  Future<List<ReviewDiscussion>> getReviewDiscussions({required String reviewId, dynamic hint}) {
    return api.getReviewDiscussions(providerId: providerId, reviewId: reviewId, hint: hint);
  }

  Future<List<ReviewFileSummary>> getReviewFileSummaries({required String reviewId, dynamic hint}) {
    return api.getReviewFileSummaries(providerId: providerId, reviewId: reviewId, hint: hint);
  }

  Future<ReviewFileChanges> getReviewFile(
      {required String reviewId,
      required String filePath,
      required String revision,
      dynamic hint}) {
    return api.getReviewFile(
        providerId: providerId,
        reviewId: reviewId,
        filePath: filePath,
        revision: revision,
        hint: hint);
  }

  Future<void> markFileRead(
      {required String reviewId,
      required String filePath,
      required String revision,
      required bool read,
      dynamic hint}) {
    return api.markFileRead(
        providerId: providerId,
        reviewId: reviewId,
        filePath: filePath,
        revision: revision,
        read: read,
        hint: hint);
  }
}

extension ProviderSettingsExtension on ProviderSettings {
  String get title {
    return module.map(
        upsource: (_) => "Upsource", github: (_) => "Github", gitlab: (_) => "Gitlab");
  }
}
