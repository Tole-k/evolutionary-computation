from utils import dill_cache
from problem import main
import streamlit as st
import evolutionary
import plotly.express as px
import pandas as pd

PSEUDOCODE = r"""
```py
    fn node_similarity(solution_a, solution_b):
        set_a = into_set(solution_a)
        set_b = into_set(solution_b)
        intersection = set_a intersect set_b
        return SIZE(intersection)
    
    fn edge_similarity(solution_a, solution_b):
        set_a = into_set(solution_a.edges)
        set_b = into_set(solution_b.edges)
        intersection = set_a intersect set_b
        return SIZE(intersection)
    
    fn similarity_tests(similarity_measure):
        solutions = []
        for i from 0 to 1000:
            solution = GENERATE_RANDOM_SOLUTION()
            solution = GREEDY_EDGES_LOCAL_SEARCH(solution)
            solutions.append(solution)
        
        very_good_solution = ILS()
        best_solution = MIN_SCORE(solutions)
        
        best_similarities = []
        very_good_similarities = []
        avg_similarities = []
        
        for solution in solutions:
            best_similarity = similarity_measure(solution, best_solution)
            very_good_similarity = similarity_measure(solution, very_good_solution)
            avg_similarity = AVERAGE([similarity_measure(solution, other) for other in solutions if other != solution])
            
            if best_solution != solution:
                best_similarities.append(best_similarity)
            very_good_similarities.append(very_good_similarity)
            avg_similarities.append(avg_similarity)
        
        scores = solutions.scores
        
        corr_best = PEARSON(best_similarities, scores.remove(best_solution.score))
        corr_very_good = PEARSON(very_good_similarities, scores)
        corr_avg = PEARSON(avg_similarities, scores)
        
    fn PEARSON(x,y):
        n = LENGTH(x)
        sum_x2 = SUM([i*i for i in x])
        sum_y2 = SUM([j*j for j in y])
        sum_xy = SUM([i*j for i,j in ZIP(x,y)])
        
        return (n * sum_xy - SUM(x) * SUM(y)) / (SQRT((n * sum_x2 - SUM(x)**2)) * (SQRT(n * sum_y2 - SUM(y)**2)))

```
"""

CONCLUSIONS = r"""
    - Solution fitness and its similarity (in all three kinds) is strongly correlated in almost all cases.
    - The better the solution (the lower its score), the higher its similarity.
    - This suggests that the bests solutions are similar to each other.
    - Avg similarity to other solutions generally shows the highest correlation.
    - Similarity to the best found solution usually shows the lowest correlation, but still significant.
    - Neither of problem instances nor similarity measures show significantly higher correlation.
    
"""


def generate_plot(tsp_version: str, measure: str):

    @dill_cache(f"similarity-{tsp_version}-{measure}")
    def get_data():
        return evolutionary.similarity_tests(tsp_version, measure)

    results, correlations = get_data()
    corr_best, corr_very_good, corr_avg = correlations

    data_x, data_y = list(zip(*results))
    data_best, data_very_good, data_avg = list(zip(*data_y))
    excluded = data_best.index(-1)
    tmp_data_x = list(data_x)
    tmp_data_best = list(data_best)
    tmp_data_best.pop(excluded)
    tmp_data_x.pop(excluded)

    col1, col2, col3 = st.columns(3)

    with col1:
        df_best = pd.DataFrame({"Cost": tmp_data_x, "Similarity": tmp_data_best})
        fig_best = px.scatter(df_best, x="Cost", y="Similarity", title=f"Similarity to best found solution (r={corr_best:.3f})")
        fig_best.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_best)

    with col2:
        df_very_good = pd.DataFrame({"Cost": data_x, "Similarity": data_very_good})
        fig_very_good = px.scatter(
            df_very_good, x="Cost", y="Similarity", title=f"Similarity to a very good solution (ILS) (r={corr_very_good:.3f})"
        )
        fig_very_good.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_very_good)

    with col3:
        df_avg = pd.DataFrame({"Cost": data_x, "Similarity": data_avg})
        fig_avg = px.scatter(df_avg, x="Cost", y="Similarity", title=f"Avg similarity to other found solutions (r={corr_avg:.3f})")
        fig_avg.update_layout(xaxis=dict(nticks=10), yaxis=dict(nticks=10))
        st.plotly_chart(fig_avg)


def report():
    main(report=True)
    st.markdown(PSEUDOCODE)
    st.image("pearson.png", width=600)
    for state in ["TSPA", "TSPB"]:
        st.header(f"Results for {state.replace('A', ' A').replace('B', ' B')}")
        for measure in ["node", "edge"]:
            st.subheader(f"{measure} similarity")
            generate_plot(state, measure)


def page():
    st.markdown(PSEUDOCODE)
    state = st.session_state.get("tsp_version")
    if state not in ["TSP A", "TSP B"]:
        raise ValueError(f"Impossible TSP state reached: {state}")
    measure = st.selectbox("Measure", ["node", "edge"])
    st.subheader(f"{measure} similarity")
    generate_plot(state.replace(" ", ""), measure)


if __name__ == "__main__":
    st.title("Similarity Tests")
    st.set_page_config(layout="wide")
    if st.session_state.get("report_mode"):
        report()
    else:
        page()
    st.subheader("Conclusions")
    st.markdown(CONCLUSIONS)
