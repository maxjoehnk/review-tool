import 'package:flutter_bloc/flutter_bloc.dart';
import 'package:review_tool/api.dart';

class RemoteList<T> {
  final List<T> _inner;
  final bool pending;

  RemoteList(this._inner, {this.pending = false});

  RemoteList<T> copyWith({List<T>? inner, bool? pending}) {
    return RemoteList(inner ?? _inner, pending: pending ?? this.pending);
  }

  List<T> get list {
    return _inner;
  }
}

class ReviewListState extends Cubit<RemoteList<Review>> {
  final ProviderApi api;

  ReviewListState(this.api) : super(RemoteList([]));

  fetchReviews() async {
    emit(state.copyWith(pending: true));
    var reviews = await api.getReviews();

    emit(state.copyWith(pending: false, inner: reviews));
  }
}
