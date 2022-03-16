import 'package:bloc/bloc.dart';
import 'package:collection/collection.dart';
import 'package:review_tool/api.dart';

class ReviewState {
  final Review review;
  final List<ReviewFileSummary> files;
  final List<ReviewDiscussion> discussions;
  final ReviewFileSummary? selectedFile;
  final bool loading;

  ReviewState(
      {required this.review,
      required this.files,
      required this.discussions,
      this.loading = false,
      this.selectedFile});

  ReviewState copyWith(
      {Review? review,
      List<ReviewFileSummary>? files,
      List<ReviewDiscussion>? discussions,
      ReviewFileSummary? selectedFile,
      bool? loading}) {
    return ReviewState(
        review: review ?? this.review,
        files: files ?? this.files,
        discussions: discussions ?? this.discussions,
        selectedFile: selectedFile ?? this.selectedFile,
        loading: loading ?? this.loading);
  }

  ReviewState setFileReadState(ReviewFileSummary file, bool isRead) {
    var files = [...this.files];
    var index = files.indexOf(file);
    files[index] = ReviewFileSummary(
      fileName: file.fileName,
      addedLines: file.addedLines,
      removedLines: file.removedLines,
      changeType: file.changeType,
      filePath: file.filePath,
      filePathSegments: file.filePathSegments,
      revisionId: file.revisionId,
      isRead: isRead,
    );
    return copyWith(files: files);
  }
}

class ReviewCubit extends Cubit<ReviewState> {
  final ProviderApi api;

  ReviewCubit(this.api, Review review)
      : super(ReviewState(review: review, files: [], discussions: []));

  fetch() async {
    emit(state.copyWith(loading: true));
    var result = await Future.wait<dynamic>([
      api.getReviewFileSummaries(reviewId: state.review.id),
      api.getReviewDiscussions(reviewId: state.review.id)
    ]);

    emit(state.copyWith(files: result[0], discussions: result[1], loading: false));
  }

  selectFile(String filePath, String revision) async {
    var file = state.files
        .firstWhereOrNull((file) => file.revisionId == revision && file.filePath == filePath);
    await api.markFileRead(
        reviewId: state.review.id, filePath: filePath, revision: revision, read: true);

    emit(state.copyWith(selectedFile: file).setFileReadState(file!, true));
  }

  toggleFileRead(ReviewFileSummary file) async {
    await api.markFileRead(
        reviewId: state.review.id,
        filePath: file.filePath,
        revision: file.revisionId,
        read: !file.isRead);
    emit(state.setFileReadState(file, !file.isRead));
  }
}
